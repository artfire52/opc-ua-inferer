#!/usr/bin/env python 
import os
import argparse

def main(configuration_file,tag_file,output_dir,limit):
    try:
        with open(configuration_file,"r") as f:
            config=f.readlines()
        with open(tag_file,"r") as f:
            tags=f.readlines()
            tags.reverse()
    except e:
        print(e)
        exit(1)
    version_line_number=-1
    implem=""
    for nb,i in enumerate(config):
        if "Version" in i:
            version_line_number=nb
            if implem!="":
                break
        if "Implem" in i:
            implem=i.split(":")[1].strip()
            if version_line_number!=-1:
                break
    if limit==None:
        limit=len(tags)
    else:
        limit=int(limit)
    for version in tags[:limit]:
        config[version_line_number]="Version: "+version
        output_file=output_dir+implem+"_"+version
        with open(output_file.strip(),"w") as f:
            f.writelines(config)



if __name__=="__main__":
    parser=argparse.ArgumentParser(description="take a conf file, a list of tag in a file and generate the same conf with all version")
    parser.add_argument("-c",help="configuration base that will be copy")
    parser.add_argument("-t",help="file with all the tag (one per line). git tag > file to have it")
    parser.add_argument("-o",help="output directory")
    parser.add_argument("-n",help="limit the number of tag")
    args=parser.parse_args()
    if args.c!=None and args.t!=None and args.o!=None:
        output_dir=args.o if args.o[-1]=="/" else args.o+"/"
        main(args.c,args.t,output_dir,args.n)
