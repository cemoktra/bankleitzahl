curl -L https://www.bundesbank.de/resource/blob/602632/873a40dd68e39a5ea7ef0a4fd41d97b4/mL/blz-aktuell-txt-data.txt > ascii.txt
iconv -f ISO-8859-1 -t UTF-8 ascii.txt > data.txt
rm ascii.txt