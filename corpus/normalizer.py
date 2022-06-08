from encodings import utf_8
from konlpy.tag import Okt
import re
import sys

eq_class = (
    ({"나라", "국가"}, "나라-국가"),
    ({"정치인", "정치가"}, "정치인-정치가"),
    ({"한국", "대한민국"}, "한국-대한민국"),
    ({"중화인민공화국", "중국"}, "중화인민공화국-중국"),
    ({"대학", "대학교"}, "대학-대학교"),
    ({"노벨상", "노벨"}, "노벨상-노벨"))

expand = (
    ("천문물리학", "물리학"),
    ("천체물리학자", "물리학자"),
    ("생물리학자", "생물학자"),
    ("생물리학", "생물학"),
    ("대도시", "도시"),
    ("여신", "신"),
)

def to_num(str):
    result = ""
    for c in str:
        if c.isnumeric():
            result += c

    return result

def normalize(str):
    result = okt.pos(str, norm=True, stem=True)
    result = list(map(lambda x: (to_num(x[0]), x[1]) if x[1] == 'Number' else x, result))

    result_tmp = list(result)
    for i in range(len(result)):
        for exp in expand:
            if result[i][0] == exp[0]:
                result_tmp.append((exp[1], result[i][1]))

    result = list(result_tmp)
    for i in range(len(result)):
        for eqc in eq_class:
            if result[i][0] in eqc[0]:
                result[i] = (eqc[1], result[i][1])

    return result

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
                    result = list(map(lambda x: x[0].strip(), normalize(line)))
                    output.write(' '.join(result))
else:
    argv = ' '.join(sys.argv[1:])
    result = list(map(lambda x: x[0].strip(), filter(lambda x: x[1] != '', normalize(argv.strip()))))
    print(' '.join(result))