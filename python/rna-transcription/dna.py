DNA_TO_RNA = {
    'C': 'G',
    'G': 'C',
    'T': 'A',
    'A': 'U',
}

def to_rna(nucleotides):
    transcription = []
    for n in nucleotides:
        transcription.append(DNA_TO_RNA[n])
    return ''.join(transcription)
