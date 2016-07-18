RNA_COMPLEMENTS = {
    'C': 'G',
    'G': 'C',
    'T': 'A',
    'A': 'U',
}

def to_rna(nucleotides):
    transcription = []
    for n in nucleotides:
        transcription.append(RNA_COMPLEMENTS[n])
    return ''.join(transcription)
