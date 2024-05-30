# opc-ua-inferer
This project uses active automata learning with OPC UA implementations:
- [S2OPC](https://gitlab.com/systerel/S2OPC)
- [Open62541](https://github.com/open62541/open62541)
- [opcua-asyncio](https://github.com/FreeOpcUa/opcua-asyncio)
- [UANET](https://github.com/OPCFoundation/UA-.NETStandard)

## Python version
The mapper is using cpython (version = "0.7") crate. So please use python version below 3.11

## Run active automata learning

```
usage: inference.py [-h] [-f path to file]
                    [-i inference id when running in parallel ignore or 0 otherwise. if the value is different from 0 the option skip-learner is set.]
                    [-o path to folder] [-c] [--skip-learner] [--skip-target]

launch inference

optional arguments:
  -h, --help            show this help message and exit
  -f path to file       filename
  -i inference id when running in parallel (ignore or 0 otherwise). if the value is different from 0 the option skip-learner is set.
                        inference id
  -o path to folder     output directory
  -c                    display how should a configuration file should be and then exit
  --skip-learner        avoid to construct the docker image when images already exists
  --skip-target         avoid to construct the docker images when images already exists

```
The *skip-x* options are mostly useful to save time when inferences crash due to timeout begin too short.
## Configuration
This is a folder that contains the configuration files to run inference.
```
usage: configuration_generator.py [-h] [-c C] [-t T] [-o O] [-n N]

take a conf file, a list of tag in a file and generate the same conf with all version

optional arguments:
  -h, --help  show this help message and exit
  -c C        configuration base that will be copy
  -t T        file with all the tag (one per line). git tag > file to obtain it
  -o O        output directory
  -n N        limit the number of tag
```
 For example run:
 ```
 python3 configuration_generator.py -c base/base_file -t file_with_tag_list -o output_directory
 ```
 The base file is a file with information about the configuration of an implementation.
 The tag file contains a list of tags for the configuration. Only one tag must be present in one line.
 The configuration files are already available.
 
## Target
This folder contains the element for the **inference.py** script to run target servers.

## Learner
This folder contains the learner of the MAT framework.
It is based on [pylstar](https://github.com/gbossert/pylstar).

## Mapper
This folder contains the mapper for the inference. It transcript abstract strings such as "hello" to OPC UA binary message. A better version of the code will be released in another repository.

## Publication

This repository is related to the publication
```
Mealy Verifier: An Automated, Exhaustive, and Explainable
Methodology for Analyzing State Machines in Protocol
Implementations
doi: 10.1145/3664476.3664506
published in: The 19th International Conference on Availability, Reliability and Security (ARES 2024), July 30-August 2, 2024, Vienna, Austria
```


