# rustlang-example

Example of Rust code in which it does what is referred to as crib dragging.

You can run the commands of:

```sh
cargo build
cargo test
cargo clippy
cargo fmt
```

And it should build, test, lint, and already be formatted.

You should be able to run the command of:

```sh
cargo run
```

And then you will see this very large text screen and prompt

```text
Welcome to the Cipher Cracker
----
This program will help you crack a cipher that has been encoded with a fatal flaw. By having an initial
piece of information such as a word or phrase, you can use this information to help crack parts of the cipher.
Text you use to try and crack the Cipher is called a 'crib' and the techniques used under the hood are XOR strings.
For example, if you know that the word 'cryptographer' is in the encoded text, you can use this word as a crib to help crack the cipher.
After each word(s) you try, a set of 'chunks' of text will be displayed to help see if you decoded a different part of the message.
If you have decoded a different part of the message you can optionally use that text or complete any word(s) or phrase to crack more of the message.
The output might have tabs, spaces, or new line characters and is expected to be messy as you try to extract information from the message.
It is expected that chunks will be displayed which are not part of the message as well.
Start by typing in the word 'cryptographer' and see what happens.
See https://samwho.dev/blog/toying-with-cryptography-crib-dragging/ for more information.
----

Avaialble Options:
Enter any ascii character for a crib try (e.x. cryptographer)
Enter 1 to print history of crib attempts you have tried so far
Enter 2 for a hint
Enter 3 to reveal the message
Enter 4 to exit
Enter option:
```

You can then enter a "crib drag" such as the word "cryptographer"

```text
Enter option: cryptographer
```

And you will see a very large dump of strings like so:

```text
0: cry"<`to5'z}|
1: cr+8{|z&6bpkgN
2: c 1gr3%shfp6]
3: 1:vci;0`y~}!%
4: +}jm 8ujoe,2o
5: lad$#}|t4?xv
6: po-'fwig%'ua{

7: ~&.blar66mllr
8: 7%khzz#%|tae`
9: 4`a~a+0oeyhwv^
10: qjwe08zvhpza&
11: {|l4#rc{abl1nE
12: mg='iknrst<y=E
13: v6.mpfg`e$t*=
14: '%dt}ouv5l'*wE
15: 4o}yt}c&}?'`=
16: ~vppfk3n.?m*h
17: g{ybp;{=.u'n
18: jrkt s(=d?ryvI
19: c`}$h (w.jta1
20: qv-l; b={ll&eX
21: g&e?;j(h}t+r
22: 7n6?q }ne37g
                 23: =6u;u{v"g:pc
24: ,=|?nsc1v"}tq
25: ,w6jhk$e3eyfqC
26: f=clp,p takf;
27: ,het7x5gpsk,b
28: yn}3c=rcbs!uz
29: v:g&zvqb9xmv
30: g1n"a~dq(``ay
31:  e+eeld;qxlnx
32: t lawl.bitcoiD *** (possible drag match)
33: 1ghsw&wze{b~<+
34: vczs=ovjzs+S
35: rqz9dgcykk&D{
36: `q0`|klxz>Il{K
37: `;ixpdmi/Qal3
38: *bqte|<@ya$b
39: sz}{~t)Shy)uqQ
40: kvrzo!F{h1xf)I
41: gysk:Nn{ `k>1
42: hxb>Ufn3qs3&|
43: ii7Q}f&bb++kmO
44: x<Xy}.wq:3fz7^
45: -Spy5d)"~w &
46: B{p1dl<1oo-1oO
47: j{8`w4$|~5<x7
48: j3is/,im$$u a

49: "bz+7ax75m-vs
50: sq"3zp"&|5{dhO
51: `):~k*3o$ci7
52: 81wo1;z7rqr }
53:  |f5 r"a`j-jk
54: mm<$i*ts{5g|e

55: |7-m1|fh$qrr
56: &&d5gn}7nie`
57: 7o<cuu"}xghwo
58: ~7jqn*hkvpzxe^
59: &axj1`~eabur&

60: psc5{vprsm1t

61: bh<mxg`|g<cr
62: y7vicouov$nex^
63: &}`gt}ze5vho&C
64: lknpfrp&gpb1;
65: zeybix3taz<,l
66: trkmc;ark$!{|
67: c`dg igx59vk|

68: qon$rom&(nfks
69: ~e-vte3;~fdzK
70: t&p~;.lo~im3
71: 7tyz &y|oq`$i
72: ers$=qi|`x)~}E
73: cx-9jaisi1sj=
74: i&0nzafz kg*hL
75: 7;g~zno3z'4O
76: *lw~ug&in?r#7i
77: }|wq|.|}.j.
78: m|xx5th={6-~

79: msq1o`(h'5
              it

80: bz8k{ }4$dct
81: k3b;u!7|ncl
82: "iv?n)"mvn{
                83: x}6j2*~gvvhc
84: l=c61
         ktgnetoC
85: ,h?5cat}yx;A
86: y4<xiallau,9

87: %7|riypm!.rn
```

The text of

```text
t lawl.bitcoiD *** (possible drag match)
```

is where you can start to see part of the message being decryped from the crib drag. Feel free to explore the other options such as getting a hint
or if you want to you can reveal the message and then take parts of the message out to crib drag those.

See https://samwho.dev/blog/toying-with-cryptography-crib-dragging/ for more information about crib dragging in general.
