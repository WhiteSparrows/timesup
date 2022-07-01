#!/usr/bin/env python
# coding=utf-8
import requests
from bs4 import BeautifulSoup
URL = 'https://www.listchallenges.com/list-of-celebrities'

data = requests.get(URL)
soup = BeautifulSoup(data.text, features="html5lib")
s = soup.find_all('div', class_='item-name')
for i in range(len(s)):
    tmp = str(s[i])[34:]
    sep = '<'
    stripped = tmp.split(sep, 1)[0]
    print(stripped)
