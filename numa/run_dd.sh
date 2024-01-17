#!/bin/bash
function run_dd_on_cpu_node_and_mem_node() {
	CPU_NODE=$1
	MEM_NODE=$2
	printf "CPU_NODE_${CPU_NODE}_MEM_NODE_${MEM_NODE}, "
	numactl -N ${CPU_NODE} -m ${MEM_NODE} dd if=/dev/zero of=/dev/null bs=64 count=100K 2>&1 | \
		tail -n 1 | tr '(' ',' | tr ')' ',' | sed -E 's/( s)|( bytes)//g' | \
		cut -d , -f 1,5
}

for i in {0..10} ; do
	run_dd_on_cpu_node_and_mem_node 0 0
	run_dd_on_cpu_node_and_mem_node 0 1
	run_dd_on_cpu_node_and_mem_node 1 0
	run_dd_on_cpu_node_and_mem_node 1 1

	run_dd_on_cpu_node_and_mem_node 1 1
	run_dd_on_cpu_node_and_mem_node 1 0
	run_dd_on_cpu_node_and_mem_node 0 1
	run_dd_on_cpu_node_and_mem_node 0 0
done
