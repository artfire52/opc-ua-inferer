#!/usr/bin/env python3
import argparse
from multiprocessing.connection import wait
import os
from time import sleep
import mapper as Map
import socket
from pylstar.LSTAR import LSTAR
from pylstar import KnowledgeTree 
from pylstar.Letter import Letter, EmptyLetter
from pylstar.ActiveKnowledgeBase import ActiveKnowledgeBase
from pylstar.tools.Decorators import PylstarLogger
from pylstar.Word import Word
from pylstar.eqtests.RandomWalkMethod import RandomWalkMethod
from pylstar.eqtests.BDistMethod import BDistMethod
from StoreHypothesis import StoreHypothesis
inference_id=0
def get_set_element(s):
    for e in s:
        break
    return e

def fill_answer_with(prefix,value,size):
        for i in range(size-len(prefix)):
            prefix.append(Letter(value))
        return Word(prefix)
class OpcUAKnowledgeBase(ActiveKnowledgeBase):
    shift=0
    def __init__(self,timeout,nb_target,namespace,node_id,idtype,value,valtype,mode,restart_server):
        super(OpcUAKnowledgeBase, self).__init__()
        self.mapper=Map.Mapper(("crypto/uaexpert_key.der","crypto/key_wrong.der","crypto/uaexpert.der","crypto/cert_wrong.der","crypto/user_cert_true.der","crypto/user_cert_wrong.der",mode))
        self.mapper.set_target_node(idtype,namespace,node_id)
        self.mapper.set_target_node_value(valtype,value)
        self.restart_server=restart_server
        for i in range(nb_target):
            dest=f"192.123.{inference_id}.{10+i}:{4840}"
            self.mapper.get_server_certificate(dest,timeout)

    def _execute_word(self, word):
        """Executes the specified word."""
        
        if word is None:
            raise Exception("Word cannot be None")
        
        self._logger.debug("Execute word '{}'".format(word))

        self.start_target()
        try:
            return self.submit_word(word)
        finally:
            self.stop_target()

    def start_target(self):
        pass

    def stop_target(self):
        pass

    #if there is an eof we do not test since the connection will close.
    #the aim is to avoid sending unusefull request.
    def get_expected_output(self,input_word: Word) :
        prefix = input_word.letters[:-1]
        while prefix:
            # print("pre",prefix,input_word)
            try:
                prefix_word = Word(letters=prefix)
                expected_output = self.knowledge_tree.get_output_word(prefix_word).letters
                if get_set_element(expected_output[-1].symbols).split(',')[-2] == "Eof":
                    return True,fill_answer_with(expected_output, "Eof,", len(input_word.letters)), []
                ret1=[]
                ret2=[]
                for index,letter in enumerate(expected_output):
                    sym=get_set_element(letter.symbols).strip()
                    ret1.append(len(get_set_element(sym).split(',')))
                    if "No resp,"==sym:
                        ret2.append(index)
                return False, ret1, ret2 
            except Exception :
                prefix.pop()
        return False,[] , []

    

    def submit_word(self, word):
        early_eof,expected,KnownNoResp=self.get_expected_output(word)
        if early_eof:
            return expected
        word_=[get_set_element(letter.symbols) for letter in word.letters]
        OpcUAKnowledgeBase.shift+=1
        if OpcUAKnowledgeBase.shift>nb_target-1:
            OpcUAKnowledgeBase.shift=0
        dest=f"192.123.{inference_id}.{10+OpcUAKnowledgeBase.shift}:{4840}"
        # dest=f"127.0.0.1:4840" # for debug purpose only

        
        res=self.mapper.submit_word(dest,word_,OpcUAKnowledgeBase.shift,timeout,expected,KnownNoResp)
        print("##\n   ",word_,"-->",res)
        if self.restart_server:
            sock=socket.create_connection((f"192.123.{inference_id}.{10+OpcUAKnowledgeBase.shift}",5555))
            sock.close()

        ret=[]
        for i in res:
            ret.append(Letter(i))
        return Word(letters=ret)


def main(outputdir,timeout,nb_target,namespace,node_id,idtype,value,valtype,mode,input_vocabulary,restart_server=False):              
    input_letter=[Letter(i) for i in input_vocabulary]
    if outputdir[-1]=="/":
        outputdir=outputdir[:-1]
    print(f"{len(input_letter)} Letter")
    print([Letter(symbol) for symbol in input_vocabulary])
    ServerBase = OpcUAKnowledgeBase(timeout,nb_target,namespace,node_id,idtype,value,valtype,mode,restart_server)
    try:
        ServerBase.start_target()
        store=StoreHypothesis(ServerBase,input_vocabulary,outputdir,BDistMethod(ServerBase,input_letter,3))
        lstar = LSTAR(input_vocabulary, ServerBase, max_states = 50,eqtests = store) #RandomWalkMethod(ServerBase, input_vocabulary, 100000, 0.7)) #
        Mapper_state_machine = lstar.learn()
    except Exception as e:
        print(e)
        ServerBase.stop_target()
        exit(45)
    finally:
        ServerBase.stop_target()
        
    dot_code = Mapper_state_machine.build_dot_code() 
    outputdir+="/"
    output_file_automata = outputdir+"automata.dot"

    with open(output_file_automata, "w") as fd:
        fd.write(dot_code)

    print("==> server Automata dumped in {}".format(output_file_automata))
    output_file_stats=outputdir+"stats"
    with open(output_file_stats, "w") as fd:
        fd.write(format(ServerBase.stats))


if __name__ == "__main__":
    
    parser=argparse.ArgumentParser(description="create docker compose file")
    parser.add_argument('-o', nargs='?',type=str,help="name of ouput directory")
    parser.add_argument('--voc',help="vocabulary as msg1,msg2...")
    parser.add_argument('--nb_target',metavar="nb target",type=int,help='number of container running at the same time')
    parser.add_argument('--node',metavar="node_id",help="node id to write or read")
    parser.add_argument('--value',metavar="value",help="value to write for the node")
    parser.add_argument('--ns',metavar="namespace",help="namespace the node",type=int)
    parser.add_argument('--idtype',metavar="IdType",help="type of identifier for the node")
    parser.add_argument('--valtype',metavar="ValType",help="type of value in the node")
    parser.add_argument('-t',metavar="timeout in ms",help="timeout in ms")    
    parser.add_argument('-m',metavar="mode",help="1 nothing, 2 signature, 3 signature + encryption")    
    parser.add_argument('--restart-server',action="store_true",help="restart the target server only available on rust target")  
    parser.add_argument('--inference-id',metavar="inference id",help="id of the inference. Used for parallel case. default value to 0. If it is not use in parallel use 0.",type=int,default=0)    
    args=parser.parse_args()
    inference_id=args.inference_id 
    if args.node!=None and args.value!=None and args.idtype!=None and args.valtype!=None and args.voc!=None and args.o!=None and args.t!=None and args.nb_target!=None and args.ns!=None and args.m!=None:
        node_id=args.node
        value=args.value
        NodeIdType=eval("Map."+args.idtype)
        ValType=eval("Map."+args.valtype)
        timeout=int(args.t)
        input_vocabulary=[i for i in args.voc.split(",")]
        outputdir=args.o
        nb_target=int(args.nb_target)
        namespace=args.ns
        mode=int(args.m)
    else:
        parser.print_help()
        exit(1)
    main(outputdir,timeout,nb_target,namespace,node_id,NodeIdType,value,ValType,mode,input_vocabulary,args.restart_server)


