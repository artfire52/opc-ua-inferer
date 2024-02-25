#!/usr/bin/env python3
import argparse



def add_containers(n,inference_id,image):
    text="services:\n"
    space="    "
    for i in range(n):
        text+=space+f"server_{inference_id}_{i}:\n"
        text+=space*2+f"image: {image}\n"
        text+=space*2+f"""command: "bash -c \\\"sed -i s/0.0.0.0/192.123.{inference_id}.{10+i}/g server-with-encryption.py &&  python3 server-with-encryption.py\\\""\n"""
        text+=space*2+f"container_name: server_{inference_id}_{i}\n"
        text+=space*2+f"restart: always\n"
        text+=space*2+f"networks:\n"
        text+=space*3+f"inference_network_{inference_id}:\n"
        text+=space*4+f"ipv4_address: 192.123.{inference_id}.{10+i}\n"
    return text

def add_network(name):
    text=""
    space="    "
    text+=f"networks:\n"
    text+=space+f"{name}:\n"
    text+=space*2+f"external: true\n"
    return text




if __name__ == "__main__":
    parser=argparse.ArgumentParser(description="create docker compose file")
    parser.add_argument('image', nargs='?',type=str,help="docker image to build container for")
    parser.add_argument('integers', nargs='?',type=int,help="number of container")
    parser.add_argument("--inference-id",metavar="inference id when running in parallel (ignore or 0 otherwise)",help="inference id",type=int,default=0)
    args=parser.parse_args()
    if args.integers!=None and args.image!=None:
        # print(args.integers)
        content="version: '3'\n"
        content+=add_containers(args.integers,args.inference_id,args.image)
        content+="\n"
        content+=add_network(f"inference_network_{args.inference_id}")
        f=open("docker-compose.yaml",'w')
        f.write(content)
        f.close()
    else:
        parser.print_help()
    
