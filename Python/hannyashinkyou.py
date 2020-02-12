#!usr/bin/python
# -*- coding: UTF-8 -*-

# PyAutoGUIのモジュール
# pip install pyautogui
import pyautogui

# クリップボードコピペ用
# pip install pyperclip
import pyperclip

import sys
import time


def GijiHenkan(kanji, roumaji, sleeptime):
    # roumaji文字列をタイプする（※全角モード前提）
    # pyautogui.typewrite(roumaji)
    # ↑不自然に早いので不採用

    # 全部の文字を一文字ずつ打つ
    for char in roumaji:
        pyautogui.press(char, presses=1)
        time.sleep(sleeptime)

    # 変換前にひとこきゅう
    time.sleep(sleeptime)

    # クリップボードに漢字をコピーしておく
    pyperclip.copy(kanji)
    # 消去した瞬間にクリップボードから文字をコピペ
    # 消去については環境ごとに異なる可能性があるが、
    # escの２回押しにしておく。
    pyautogui.press('esc', presses=2)
    # コピペ
    pyautogui.hotkey('ctrl', 'v')
    # ひとこきゅう
    time.sleep(sleeptime)

    return 0


def Kaigyou(sleeptime):
    pyautogui.press('enter', presses=1)
    time.sleep(sleeptime)
    return 0


def DoSyakyou(sleeptime, kaigyousleeptime):
    GijiHenkan("摩", "ma", sleeptime)
    GijiHenkan("訶", "ka", sleeptime)
    GijiHenkan("般", "hann", sleeptime)
    GijiHenkan("若", "nya", sleeptime)
    GijiHenkan("波", "ha", sleeptime)
    GijiHenkan("羅", "ra", sleeptime)
    GijiHenkan("蜜", "mi", sleeptime)
    GijiHenkan("多", "ta", sleeptime)
    GijiHenkan("心", "sinn", sleeptime)
    GijiHenkan("経", "gyou", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("観", "kann", sleeptime)
    GijiHenkan("自", "ji", sleeptime)
    GijiHenkan("在", "zai", sleeptime)
    GijiHenkan("菩", "bo", sleeptime)
    GijiHenkan("薩", "satu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("行", "gyou", sleeptime)
    GijiHenkan("深", "jinn", sleeptime)
    GijiHenkan("般", "hann", sleeptime)
    GijiHenkan("若", "nya", sleeptime)
    GijiHenkan("波", "ha", sleeptime)
    GijiHenkan("羅", "ra", sleeptime)
    GijiHenkan("蜜", "mixtu", sleeptime)
    GijiHenkan("多", "ta", sleeptime)
    GijiHenkan("時", "ji", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("照", "syou", sleeptime)
    GijiHenkan("見", "ken", sleeptime)
    GijiHenkan("五", "go", sleeptime)
    GijiHenkan("蘊", "unn", sleeptime)
    GijiHenkan("皆", "kai", sleeptime)
    GijiHenkan("空", "kuu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("度", "do", sleeptime)
    GijiHenkan("一", "ixtu", sleeptime)
    GijiHenkan("切", "sai", sleeptime)
    GijiHenkan("苦", "ku", sleeptime)
    GijiHenkan("厄", "yaku", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("舍", "sya", sleeptime)
    GijiHenkan("利", "ri", sleeptime)
    GijiHenkan("子", "si", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("色", "siki", sleeptime)
    GijiHenkan("不", "hu", sleeptime)
    GijiHenkan("異", "i", sleeptime)
    GijiHenkan("空", "kuu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("空", "kuu", sleeptime)
    GijiHenkan("不", "hu", sleeptime)
    GijiHenkan("異", "i", sleeptime)
    GijiHenkan("色", "siki", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("色", "siki", sleeptime)
    GijiHenkan("即", "soku", sleeptime)
    GijiHenkan("是", "ze", sleeptime)
    GijiHenkan("空", "kuu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("空", "kuu", sleeptime)
    GijiHenkan("即", "soku", sleeptime)
    GijiHenkan("是", "ze", sleeptime)
    GijiHenkan("色", "siki", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("受", "jyu", sleeptime)
    GijiHenkan("想", "sou", sleeptime)
    GijiHenkan("行", "gyou", sleeptime)
    GijiHenkan("識", "siki", sleeptime)
    GijiHenkan("亦", "yaku", sleeptime)
    GijiHenkan("復", "bu", sleeptime)
    GijiHenkan("如", "nyo", sleeptime)
    GijiHenkan("是", "ze", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("舍", "sya", sleeptime)
    GijiHenkan("利", "ri", sleeptime)
    GijiHenkan("子", "si", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("是", "ze", sleeptime)
    GijiHenkan("諸", "syo", sleeptime)
    GijiHenkan("法", "hou", sleeptime)
    GijiHenkan("空", "kuu", sleeptime)
    GijiHenkan("相", "sou", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("不", "hu", sleeptime)
    GijiHenkan("生", "syou", sleeptime)
    GijiHenkan("不", "hu", sleeptime)
    GijiHenkan("滅", "metu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("不", "hu", sleeptime)
    GijiHenkan("垢", "ku", sleeptime)
    GijiHenkan("不", "hu", sleeptime)
    GijiHenkan("浄", "jyou", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("不", "hu", sleeptime)
    GijiHenkan("増", "zou", sleeptime)
    GijiHenkan("不", "hu", sleeptime)
    GijiHenkan("減", "genn", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("是", "ze", sleeptime)
    GijiHenkan("故", "ko", sleeptime)
    GijiHenkan("空", "kuu", sleeptime)
    GijiHenkan("中", "tyuu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("色", "siki", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("受", "jyu", sleeptime)
    GijiHenkan("想", "sou", sleeptime)
    GijiHenkan("行", "gyou", sleeptime)
    GijiHenkan("識", "siki", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("眼", "genn", sleeptime)
    GijiHenkan("耳", "ni", sleeptime)
    GijiHenkan("鼻", "bi", sleeptime)
    GijiHenkan("舌", "zextu", sleeptime)
    GijiHenkan("身", "sinn", sleeptime)
    GijiHenkan("意", "i", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("色", "siki", sleeptime)
    GijiHenkan("声", "syou", sleeptime)
    GijiHenkan("香", "kou", sleeptime)
    GijiHenkan("味", "mi", sleeptime)
    GijiHenkan("触", "soku", sleeptime)
    GijiHenkan("法", "hou", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("眼", "genn", sleeptime)
    GijiHenkan("界", "kai", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("乃", "nai", sleeptime)
    GijiHenkan("至", "si", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("意", "i", sleeptime)
    GijiHenkan("識", "siki", sleeptime)
    GijiHenkan("界", "kai", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("明", "myou", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("亦", "yaku", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("明", "myou", sleeptime)
    GijiHenkan("尽", "jinn", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("乃", "nai", sleeptime)
    GijiHenkan("至", "si", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("老", "rou", sleeptime)
    GijiHenkan("死", "si", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("亦", "yaku", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("老", "rou", sleeptime)
    GijiHenkan("死", "si", sleeptime)
    GijiHenkan("尽", "jinn", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("苦", "ku", sleeptime)
    GijiHenkan("集", "syuu", sleeptime)
    GijiHenkan("滅", "metu", sleeptime)
    GijiHenkan("道", "dou", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("智", "ti", sleeptime)
    GijiHenkan("亦", "yaku", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("得", "toku", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("以", "i", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("所", "syo", sleeptime)
    GijiHenkan("得", "toku", sleeptime)
    GijiHenkan("故", "kou", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("菩", "bo", sleeptime)
    GijiHenkan("提", "dai", sleeptime)
    GijiHenkan("薩", "saxtu", sleeptime)
    GijiHenkan("埵", "ta", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("依", "e", sleeptime)
    GijiHenkan("般", "hann", sleeptime)
    GijiHenkan("若", "nya", sleeptime)
    GijiHenkan("波", "ha", sleeptime)
    GijiHenkan("羅", "ra", sleeptime)
    GijiHenkan("蜜", "mixtu", sleeptime)
    GijiHenkan("多", "ta", sleeptime)
    GijiHenkan("故", "ko", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("心", "sinn", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("罣", "kei", sleeptime)
    GijiHenkan("礙", "ge", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("罣", "kei", sleeptime)
    GijiHenkan("礙", "ge", sleeptime)
    GijiHenkan("故", "ko", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("有", "u", sleeptime)
    GijiHenkan("恐", "ku", sleeptime)
    GijiHenkan("怖", "hu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("遠", "onn", sleeptime)
    GijiHenkan("離", "ri", sleeptime)
    GijiHenkan("一", "ixtu", sleeptime)
    GijiHenkan("切", "sai", sleeptime)
    GijiHenkan("顛", "tenn", sleeptime)
    GijiHenkan("倒", "dou", sleeptime)
    GijiHenkan("夢", "mu", sleeptime)
    GijiHenkan("想", "sou", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("究", "ku", sleeptime)
    GijiHenkan("竟", "kyou", sleeptime)
    GijiHenkan("涅", "ne", sleeptime)
    GijiHenkan("槃", "hann", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("三", "sann", sleeptime)
    GijiHenkan("世", "ze", sleeptime)
    GijiHenkan("諸", "syo", sleeptime)
    GijiHenkan("仏", "butu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("依", "e", sleeptime)
    GijiHenkan("般", "hann", sleeptime)
    GijiHenkan("若", "nya", sleeptime)
    GijiHenkan("波", "ha", sleeptime)
    GijiHenkan("羅", "ra", sleeptime)
    GijiHenkan("蜜", "mixtu", sleeptime)
    GijiHenkan("多", "ta", sleeptime)
    GijiHenkan("故", "ko", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("得", "toku", sleeptime)
    GijiHenkan("阿", "a", sleeptime)
    GijiHenkan("耨", "noku", sleeptime)
    GijiHenkan("多", "ta", sleeptime)
    GijiHenkan("羅", "ra", sleeptime)
    GijiHenkan("三", "sann", sleeptime)
    GijiHenkan("藐", "myaku", sleeptime)
    GijiHenkan("三", "sann", sleeptime)
    GijiHenkan("菩", "bo", sleeptime)
    GijiHenkan("提", "dai", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("故", "ko", sleeptime)
    GijiHenkan("知", "ti", sleeptime)
    GijiHenkan("般", "hann", sleeptime)
    GijiHenkan("若", "nya", sleeptime)
    GijiHenkan("波", "ha", sleeptime)
    GijiHenkan("羅", "ra", sleeptime)
    GijiHenkan("蜜", "mixtu", sleeptime)
    GijiHenkan("多", "ta", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("是", "ze", sleeptime)
    GijiHenkan("大", "dai", sleeptime)
    GijiHenkan("神", "jinn", sleeptime)
    GijiHenkan("呪", "syu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("是", "ze", sleeptime)
    GijiHenkan("大", "dai", sleeptime)
    GijiHenkan("明", "myou", sleeptime)
    GijiHenkan("呪", "syu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("是", "ze", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("上", "jyou", sleeptime)
    GijiHenkan("呪", "syu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("是", "ze", sleeptime)
    GijiHenkan("無", "mu", sleeptime)
    GijiHenkan("等", "tou", sleeptime)
    GijiHenkan("等", "dou", sleeptime)
    GijiHenkan("呪", "syu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("能", "nou", sleeptime)
    GijiHenkan("除", "jyo", sleeptime)
    GijiHenkan("一", "ixtu", sleeptime)
    GijiHenkan("切", "sai", sleeptime)
    GijiHenkan("苦", "ku", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("真", "sinn", sleeptime)
    GijiHenkan("実", "jitu", sleeptime)
    GijiHenkan("不", "hu", sleeptime)
    GijiHenkan("虚", "ko", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("故", "ko", sleeptime)
    GijiHenkan("説", "setu", sleeptime)
    GijiHenkan("般", "hann", sleeptime)
    GijiHenkan("若", "nya", sleeptime)
    GijiHenkan("波", "ha", sleeptime)
    GijiHenkan("羅", "ra", sleeptime)
    GijiHenkan("蜜", "mixtu", sleeptime)
    GijiHenkan("多", "ta", sleeptime)
    GijiHenkan("呪", "syu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("即", "soku", sleeptime)
    GijiHenkan("説", "setu", sleeptime)
    GijiHenkan("呪", "syu", sleeptime)
    GijiHenkan("曰", "watu", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("羯", "gya", sleeptime)
    GijiHenkan("諦", "tei", sleeptime)
    GijiHenkan("羯", "gya", sleeptime)
    GijiHenkan("諦", "tei", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("波", "ha", sleeptime)
    GijiHenkan("羅", "ra", sleeptime)
    GijiHenkan("羯", "gya", sleeptime)
    GijiHenkan("諦", "tei", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("波", "ha", sleeptime)
    GijiHenkan("羅", "ra", sleeptime)
    GijiHenkan("僧", "sou", sleeptime)
    GijiHenkan("羯", "gya", sleeptime)
    GijiHenkan("諦", "tei", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("菩", "bo", sleeptime)
    GijiHenkan("提", "ji", sleeptime)
    GijiHenkan("薩", "so", sleeptime)
    GijiHenkan("婆", "wa", sleeptime)
    GijiHenkan("訶", "ka", sleeptime)
    Kaigyou(kaigyousleeptime)

    GijiHenkan("般", "hann", sleeptime)
    GijiHenkan("若", "nya", sleeptime)
    GijiHenkan("心", "sinn", sleeptime)
    GijiHenkan("経", "gyou", sleeptime)
    Kaigyou(kaigyousleeptime)
    Kaigyou(kaigyousleeptime)
    Kaigyou(kaigyousleeptime)

    return 0


# 以下、メインルーチン
if __name__ == "__main__":

    # 実行前の待機(秒)
    print("5秒後に写経が始まります。")
    print("心を静かにして")
    print("テキストエディタを開いて、")
    print("日本語入力モードにしておきましょう。")
    time.sleep(5)

    sleeptime = 0.0015
    kaigyousleeptime = 0.02

    # 写経開始
    # 強制終了（ctrl+c）するときにキーボード入力が継続されてしまうので
    # 実行する際には注意。
    # 繰り返し回数は最初は１回だけなどにしておいた方がよい
    for var in range(0, 1):
        DoSyakyou(sleeptime, kaigyousleeptime)

    print("おつとめおつかれさまでした。")
    sys.exit()
