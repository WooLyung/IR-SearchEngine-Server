from encodings import utf_8
from konlpy.tag import Okt
import re
import sys

okt = Okt()
sys.stdout.reconfigure(encoding='utf-8')

if len(sys.argv) == 1:
    with open('corpus.txt', 'r', encoding='utf8') as file:
        with open('nor_corpus.txt', 'w', encoding='utf8') as output:
            lines = file.readlines()
            first = True

            for line in lines:
                if re.search('<title>.*</title>', line):
                    segment = line.replace('<title>', '').replace('</title>', '').strip().split('. ')
                    docid = segment[0]
                    title = segment[1]

                    output.write(('' if first else '\n') + docid + ' ' + title + ' ')
                    first = False

                elif line.strip() != '':
                    result = list(map(lambda x: x[0].strip(), okt.pos(line, norm=True, stem=True)))
                    output.write(' '.join(result))
else:
    argv = ' '.join(sys.argv[1:])
    result = list(map(lambda x: x[0].strip(), filter(lambda x: x[1] != 'Josa', okt.pos(argv.strip(), norm=True, stem=True))))
    print(' '.join(result))