#!/usr/bin/env sh
# Example commands. Requires brrrr and jq to be on your path.

echo 'Example with fasta.'
echo
echo '>id description
ATCGATCG
>id2 desc2
CCGGCCGG' | tee /tmp/fasta.fasta

brrrr fa2jsonl /tmp/fasta.fasta | jq

echo
echo 'Example with gff'
echo
echo 'X7	AUGUSTUS	intron	1	102	1	+	.	transcript_id "file_1_file_1_g1.t1"; gene_id "file_1_file_1_g1";' | tee /tmp/gff.gtf

brrrr gff2jsonl --gff-type gff2 /tmp/gff.gtf | jq

echo 
echo 'Example FASTQ'
echo
echo '@THE_BEST_SEQ
GGGGGTT
+
!(((***' | tee /tmp/example.fastq

echo
brrrr fq2jsonl /tmp/example.fastq | jq
