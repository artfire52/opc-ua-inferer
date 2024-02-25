#!/usr/bin/env python3
import argparse
import os
import subprocess
from time import sleep
DIR=os.path.dirname(os.path.realpath(__file__))
os.chdir(DIR)
#number to identify an inference (useful in order to perform serveral inference in parallel)
inference_id=0
def parse_file(filepath):
    try:
        with open(filepath,'r') as f:
            lines=f.readlines()
    except Exception as e:
        print(e)
        exit(2)
    param=dict()
    for i in lines:
        line=i.split(':')
        if len(line)!=2:
            break
        param[line[0].strip()]=line[1].strip()
    return param


def learner_docker(param):
    """ delete old images and containers to build new one."""
    os.system(f"""docker rmi -f "learner:OPCUA"   >/dev/null """)
    subprocess.Popen(f"""docker build -t "learner:OPCUA" .  """,shell=True).wait()

def network_docker(inference_id):
    """Create docker network"""
    os.system(f"""docker network rm inference_network_{inference_id} """)
    os.system(f"""docker network create --subnet 192.123.{inference_id}.0/24 inference_network_{inference_id}""")

def target_docker(param,inference_id):
    #change directory to be able to launch build_docker_compose
    os.chdir(os.path.dirname(__file__)+"/target/server/secure/"+param["Implem"])
    os.system(f"""docker build --build-arg VersionSUT={param["Version"]} -t "inferer_{param["Implem"]}:{param["Version"]}" .  """)
    os.system(f"""./build_docker_compose.py inferer_{param["Implem"]}:{param["Version"]} {param["NbContainers"]} --inference-id {inference_id}""")
    nb_container_started=0
    nb_container_started_before=int(subprocess.check_output("docker ps -a -q | wc -l",stderr=subprocess.STDOUT,shell=True))
    subprocess.Popen(f"""docker-compose up > /dev/null""",shell=True)
    while nb_container_started-nb_container_started_before!=int(param["NbContainers"]):
        sleep(0.1)
        nb_container_started=int(subprocess.check_output("docker ps -a -q | wc -l",stderr=subprocess.STDOUT,shell=True))
    sleep(30)
    os.chdir(DIR)

def launch_target(param,inference_id):
    #change directory to be able to launch build_docker_compose
    os.chdir(os.path.dirname(__file__)+"/target/server/secure/"+param["Implem"])
    os.system(f"""./build_docker_compose.py inferer_{param["Implem"]}:{param["Version"]} {param["NbContainers"]} --inference-id {inference_id} """)
    nb_container_started=0
    nb_container_started_before=int(subprocess.check_output("docker ps -a -q | wc -l",stderr=subprocess.STDOUT,shell=True))
    subprocess.Popen(f"""docker-compose up > /dev/null""",shell=True)
    while nb_container_started-nb_container_started_before!=int(param["NbContainers"]):
        sleep(0.1)
        nb_container_started=int(subprocess.check_output("docker ps -a -q | wc -l",stderr=subprocess.STDOUT,shell=True))
    sleep(5)
    os.chdir(DIR)

def detect_failure(filepath):
    try:
        with open(filepath,'r') as f:
            content=f.readlines()
            for line in content:
                line=line.strip()
                if line[0]!= "#" and line[0] != "[" and line[0] != "=" and line[0] != '1':
                    return True
        return False
    except:
        return False

def main(filepath,outputpath,skip_target,skip_learner,inference_id,restart_server):
    param=parse_file(filepath)
    if outputpath==None:
        outputpath="result/"+param["Implem"]+"."+param["Version"]+"mode_"+param["Mode"]
        if not os.path.exists(outputpath):
            os.makedirs(outputpath)
    if outputpath==".":
        outputpath=""
    if outputpath[-1]=="/":
        outputpath=outputpath[:-1]
    os.chdir(os.path.dirname(__file__))
    if os.path.exists(outputpath+"/automata.dot"):
        print("directory already full of results")
        exit(1)
    network_docker(inference_id)
    if skip_target:
        launch_target(param,inference_id)
    else:
        target_docker(param,inference_id)
    if not skip_learner :
        learner_docker(param)
    log_output=os.path.realpath(os.path.dirname(__file__))+"/"+outputpath
    if restart_server:
        restart_server_option="--restart-server"
    else:
        restart_server_option=""
    os.system(f""" nohup docker run --name learner_{inference_id} -v $(pwd)/result:/learner/result --network inference_network_{inference_id} --ip 192.123.{inference_id}.250 "learner:OPCUA"  ./OpcUaLeaner.py  -o {outputpath} --voc {param["Vocabulary"]} -m {param["Mode"]} --nb_target {param["NbContainers"]} --node {param["NodeId"]} --value {param["Value"]} --ns {param["Namespace"]} --idtype {param["NodeIdType"]} --valtype {param["ValueType"]} -t {param["Timeout"]} {restart_server_option} --inference-id {inference_id} > {log_output}/learner_ongoing 2>&1 & """)
    while not os.path.exists(outputpath+"/automata.dot"):
        if detect_failure(log_output+"/learner_ongoing"):
            break
        sleep(10)

    os.system(f"""docker ps -a --filter "name=learner_{inference_id}" -q |xargs docker rm -f""")
    os.system(f"""docker ps -a --filter "name=server_{inference_id}_*" -q|xargs docker rm -f""")
    if not skip_target:
        os.system("docker images -f \"dangling=true\" -q | xargs docker rmi -f")
        os.system(f"""docker images -f \"reference=inferer_{param["Implem"]}:{param["Version"]}\" -q | xargs docker rmi -f""")
    with open(filepath,'r') as f:
        lines=f.readlines()
    with open(outputpath+"/configuration",'w') as configuration_file:
        configuration_file.writelines(lines)


if __name__=="__main__":
    parser=argparse.ArgumentParser(description="launch inference")
    parser.add_argument("-f",metavar="path to file",help="filename")
    parser.add_argument("-i",metavar="inference id when running in parallel (ignore or 0 otherwise). if the value is different from 0 the option skip-learner is set.",help="inference id",type=int,default=0)
    parser.add_argument("-o",metavar="path to folder",help="output directory")
    parser.add_argument("-c",action="store_true",help="display how should a configuration file should be and then exit")
    parser.add_argument("--skip-learner",action="store_true",help="avoid to construct the docker image  when images already exists")
    parser.add_argument("--skip-target",action="store_true",help="avoid to construct the docker images when images already exists")
    parser.add_argument("-r",action="store_true",help="restart the target server after each sequence.")
    args=parser.parse_args()
    if args.c:
        print("configuration file are text file written as:")
        print("param1:value1")
        print("You can put the white space you want. Order is not important either")
        print("You need to provide those parameter:")
        print("Implem: the name of the implemention you want to infer")
        print("Version: The version of the target (tag or commit)")
        print("Vocabulary: word1,word2,... (the vocabulary of the inference")
        print("NodeId: nodeid (for read and write request)")
        print("NodeIdType: type of node id (NodeIdNumeric, NodeIdString,NodeIdGuid or NodeIdByteString)")
        print("Namespace: the namespace of the node")
        print("Value: value to write")
        print("ValueType: type of the value (int8,uint8,int16,uint16,int32,uint32,float,double)")
        print("NbContainers: number of targets container")
        print("Timeout:timeout in ms (socket timeout)")
        print("Mode: encryption mode (1:nothing,2:signature,3:signature+encryption)")
        exit(0)
    if args.o:
        outputpath=args.o
    else:
        outputpath=None
    
    inference_id=args.i
    args.skip_learner= args.skip_learner if inference_id==0 else True
    if args.f:
        filepath=args.f
        main(filepath,None,args.skip_target,args.skip_learner,inference_id,args.r)
        exit(0)
    else:
        print("you have to specify a configuration file")
        parser.print_help()
        exit(1)
