use std::{cmp::{min, max}, collections::HashMap};

const INPUT: &'static str = "xhs{m>3771:A,a>2552:R,A}
smp{a<3732:R,m>1999:R,x<925:R,R}
zkk{m<2240:R,a>3345:R,R}
nqz{x<2056:R,a<3165:R,m>1748:A,vx}
mm{x>486:A,m>3523:R,A}
bg{s>3338:A,kgm}
qf{x>537:gqx,bqz}
lv{s<2341:sc,s>2505:rtm,xqj}
bv{a>2574:rvm,ncd}
ssz{a<729:A,m<3390:ld,m<3772:A,R}
gm{a>3900:A,m>2508:R,x>744:R,A}
ts{m<642:R,m<843:gmm,s<325:gf,A}
rtm{x<2920:A,m>3132:R,R}
qgz{x>2082:hvx,a<1490:A,s<3742:R,kks}
kpf{m<2390:cm,ft}
ghn{a>2066:A,s<3291:R,a>2028:A,R}
mkx{s<975:R,s<1112:A,m>756:A,R}
tg{x<3668:A,x<3835:A,ftz}
chj{x>733:R,a>3064:R,qs}
ndl{x>2773:R,x<2370:R,R}
dsq{s>838:A,s>496:A,m<2191:R,R}
jl{m>1219:A,R}
chh{x<973:R,A}
fxv{x<2543:R,m>3368:A,s<3567:R,R}
qvl{m>1648:A,R}
fpb{a>1367:R,x>2441:R,a<1220:A,R}
tph{a<2677:tss,R}
cnc{s>1274:A,a<3947:R,A}
cz{m>2865:R,a<3514:rm,s>687:xsr,A}
zzq{x>76:A,A}
vp{x<162:A,R}
bth{x<1859:zmq,a>2455:hq,mb}
rm{x<1253:A,R}
ndq{s<390:R,x<2982:A,m>3468:R,R}
vdh{x>2464:bx,m>3276:fj,ffs}
bjc{a>3069:R,R}
sqk{m>3029:R,s>130:R,sln}
xl{x<2157:A,R}
xrs{s>3440:qgz,a<1479:qsv,xt}
dnt{x<538:qc,m<1343:jqh,x>645:fm,jvb}
pms{a>2603:R,A}
lf{x>390:A,s>1617:A,A}
dvm{s>1052:R,R}
mnc{x>2703:R,A}
rxt{a<595:A,R}
pb{s<3737:ngp,x>3125:A,dbk}
zr{a<2605:R,x<422:R,A}
pd{s>1915:xhs,zpl}
vm{a>2377:R,R}
in{x<1629:dv,nfq}
xdt{s<1791:fpb,chl}
gg{s<393:fd,x<2091:dg,tz}
xsr{s>1167:R,m>2006:A,a<3531:A,R}
qs{x<719:R,x>726:R,s>2919:A,R}
kgm{m>2140:R,s<3253:R,m>1246:A,A}
gvv{s<3124:flg,m>3004:jxl,x>2411:vf,hfg}
nv{m<3088:A,a>1588:R,x>3499:A,R}
lx{s<3277:R,a>2880:R,A}
cls{m>3314:gt,kpv}
ct{a>3320:R,a>3189:A,s<2328:R,R}
bqz{m<1572:rjv,x<437:R,s>1572:mzn,td}
qm{a<1975:A,s>623:R,A}
zx{m<3520:R,s>1981:R,a<2633:A,R}
hbk{a<1441:A,x<2151:R,x>2450:A,A}
zp{a>2928:A,R}
njc{x>225:R,a>3543:A,a>3524:A,A}
dt{s<467:R,a>3868:R,x<1448:A,R}
qlj{a<1744:A,s>1375:A,R}
ln{x<546:dk,x<1088:jm,s>996:sdr,grf}
zxc{x>1220:R,a<1570:R,a<1721:R,A}
hqv{x<1168:A,a<3325:cs,zkk}
cfx{m<1777:R,A}
tb{a>3324:A,a<3310:R,R}
fx{a>2975:R,x>2219:R,A}
rnm{m<3699:A,a>1555:R,R}
zpl{a>2213:R,m<3703:A,R}
zvm{s<3648:R,x>2112:R,s>3879:A,A}
gkt{x>1247:A,m>2344:A,m<1950:R,R}
qfz{x<37:A,A}
dq{a<2482:A,A}
gpj{m>2417:A,m>1980:A,tq}
khx{s>3504:R,a>633:A,s<3211:R,A}
kh{m>629:A,A}
zg{a<2645:mg,s<259:sqk,x<2942:gg,md}
sxj{m<1304:xlx,lh}
lh{m<1742:bb,x<2813:rh,m>2090:qhd,bv}
jxg{a>3806:A,R}
sg{a<3432:nn,m<1550:pp,m<3113:ps,js}
pck{m<1755:dmv,s>3255:hb,s>2954:lfz,ns}
xqj{x>2860:A,A}
hm{a<3006:R,A}
bbv{a>3355:rf,s<1113:hqv,m>1976:qzk,zxk}
rpd{s<3371:R,A}
jzc{s>1320:A,s>1117:R,s>948:R,R}
mjp{m<3107:db,x<3329:lsq,x>3386:jgq,R}
chl{x>3157:A,m>2688:zq,tf}
ffq{x<2660:hbk,s<2231:cvs,A}
pv{x>662:bc,m>2737:fn,scz}
rf{x>1195:A,x<1012:R,hp}
mg{s<320:mnc,ndq}
md{s<368:ks,a<2900:A,a>2968:A,hxd}
rb{s<976:R,A}
cvv{s<1568:R,s>2041:A,a>2529:R,R}
hvx{s>3628:A,m>1706:R,m>1033:R,R}
fhg{m>3499:R,a>2802:nm,R}
cm{a>793:gnt,s<342:vg,dl}
mzt{m<1547:R,A}
cf{s>3282:A,R}
vr{m>1574:zzp,cfl}
vbd{x<446:R,m<2633:R,A}
mdz{a>2230:A,A}
dck{s<2674:R,m>1804:A,m>1642:A,A}
dk{x>356:A,R}
sxr{m>1197:vq,qzn}
sl{a>3532:R,A}
rhf{x<592:A,m<3490:A,A}
hp{a<3393:A,a<3436:R,x<1078:A,R}
sln{m>2771:A,R}
hq{m>3351:A,x>1966:A,m<2756:R,R}
bm{m<2254:R,hpt}
vt{m<3667:R,x<1031:A,A}
dv{a<2349:npz,a>3297:tck,x<703:gq,lm}
ddj{m<1813:mfd,A}
fln{s>2720:A,R}
xft{a<198:A,a<267:R,m>3094:A,A}
xdr{s<2440:R,m<1370:jcs,a<1346:rrl,R}
hxc{s>3518:A,a>1208:R,x>3051:R,R}
bc{a>3542:zb,s>1446:sr,a<3492:pjm,cz}
lfz{a>3921:A,m>2651:A,s>3077:qj,A}
qmk{m>2209:A,R}
bt{m>3699:R,x<2709:R,m>3441:A,A}
cfn{x<203:R,x<238:R,x>260:R,R}
hbb{m>3171:R,R}
xz{a<1217:A,m<3348:R,R}
tz{a<2930:A,A}
cvj{x>3663:A,R}
kb{x<812:A,x<882:R,m>1354:R,R}
fgc{a<3044:A,m<3797:A,a>3181:R,A}
gz{a>2501:R,x>662:R,A}
gs{m>2349:pbh,xc}
cxt{m<648:A,R}
qr{s>3439:A,s<3200:A,s>3359:R,R}
mvk{x<310:A,A}
rfx{a<3471:pb,jrp}
df{a>2425:R,a>2173:A,m<2169:A,A}
lkv{x<1956:R,m>2601:jvg,zvm}
fgv{a>3335:R,a>2798:hck,pqd}
tn{a>3644:R,m>3271:R,R}
vhl{m>3441:pd,lt}
lg{x>214:xjz,jmz}
dkl{m>979:R,x>271:R,a<2808:A,R}
pdp{s>2793:R,A}
jjf{a>3211:R,A}
tv{s<2116:sx,xsq}
gnt{m>1411:R,x<2631:A,x>3267:A,R}
lnj{a>2864:R,A}
jd{x<275:cfn,s>860:jzc,mvk}
zzd{x<1338:A,a<3957:R,A}
xlx{m<789:jqs,dpd}
hql{s<2258:cb,x>815:knq,x>758:vlq,chj}
qsv{m<2600:A,cg}
bb{a>2571:rth,m>1465:A,x<3017:R,cp}
dz{a>2559:gs,qf}
ps{s>3207:R,gkt}
mf{x<611:R,A}
gp{m>2489:zdd,x>227:R,gnc}
hv{a<3335:R,a<3515:A,R}
klr{x<295:A,m>520:R,m>218:A,A}
zq{x>2372:A,R}
zf{a>1674:R,cn}
lm{x<1313:hgj,txn}
npz{s>1686:rt,kt}
qdh{a<2898:R,x>2218:R,A}
pz{x<2285:A,A}
hdf{m<2720:A,A}
bmc{x>3617:rcl,hsb}
zh{s<1038:A,a>2811:R,R}
zfz{x>2711:R,A}
sgr{a>3615:R,m>1350:jjj,a<3585:A,A}
qvk{s>2967:R,m>3342:A,A}
pqd{s<99:A,s<208:A,x<2116:R,R}
tbd{x>3755:A,a>441:R,A}
nn{x>1301:fln,pdp}
hxd{m>3170:R,m<2811:R,x<3367:R,R}
fk{x>3114:nsg,dmq}
vzf{x>2720:ms,s>481:mhr,s<294:fgv,nqz}
lnn{s<2149:R,x>3515:tbd,a<342:xft,zcz}
tss{s>3258:A,A}
sh{a>1883:qm,m>3431:A,chx}
qj{a<3876:A,x>836:R,R}
vl{s<2783:R,m<1099:R,x<1473:A,A}
bkt{a>3162:A,s>3117:A,x>215:A,A}
jh{x>101:R,s<1379:A,s<1659:R,R}
zcp{s<611:ff,qkd}
cr{s>772:kr,m<1018:lxl,vzf}
ms{s>407:zt,x>3417:R,chq}
jm{s<1028:A,R}
bn{m<3021:vzz,m<3604:qvk,x<271:fgc,R}
ngl{s>3536:sbr,s>3350:qdh,A}
zsf{m>1545:A,x>1434:R,a<3918:R,A}
ggp{s>585:R,R}
kr{m<1070:vfc,zp}
kks{a>1629:R,a<1563:R,a<1592:A,R}
btt{a>801:A,R}
jsj{s>3393:A,x<3185:R,A}
zgx{m<855:xl,A}
gt{x<259:R,A}
gqx{x<621:A,a<2443:hx,gz}
gnk{a<2992:R,R}
vc{m>2396:xx,cl}
hhp{m>1027:sgr,ph}
chr{x<2230:R,x<2596:R,R}
qkd{x>797:A,x<297:A,s<793:R,A}
zsv{x>3189:A,s>3280:pz,x<2182:A,jxg}
bmz{x>286:A,R}
rcl{s<3551:R,a>1235:R,A}
sxl{a<1497:R,x>1555:A,R}
xmv{x<2697:shm,m<2582:bl,st}
fd{a<2818:R,R}
jgq{x<3411:R,R}
zj{a<2947:A,R}
mgb{x>2219:A,s>580:A,R}
jjs{m>2508:pj,smp}
mfd{s>3169:R,x>3872:R,R}
kpn{a<3544:R,a>3555:R,R}
lq{m>3447:A,A}
bfq{s>1868:A,qpt}
hlc{m>3423:rnm,a<1561:A,nv}
zn{x<915:A,m<570:A,a>3454:A,A}
zmq{a>2577:A,s<2162:R,a<2036:R,R}
sqz{s<1195:A,R}
jsd{m<1634:vs,R}
dmq{m<1655:A,A}
nhl{a<2744:A,A}
mkm{s<482:A,s>608:A,R}
rc{m>234:A,x>2208:R,m>97:R,A}
zz{a>3346:vlc,a<2753:qqc,s>276:fx,rc}
js{a>3452:R,m<3619:A,R}
tj{a>1494:R,m<658:R,R}
qhd{s<2443:mdz,znr}
shv{m<2846:A,x>2878:A,A}
ttn{a>3182:R,a>3140:R,x<475:R,R}
ngp{x>3141:A,A}
pcn{x<1006:R,s>520:A,A}
ngq{m<1256:R,s>1251:R,a>658:R,R}
fkd{a<3655:dd,a<3774:cfx,s>2066:qmk,fk}
qzk{a>3335:R,R}
dx{m<2588:zcp,a<2083:sh,x<601:rrd,fh}
qc{x<432:zh,s>956:R,A}
fm{s<642:R,m>2905:lq,dcp}
rr{s>2991:xmv,ths}
dcp{a>2785:A,s<1372:R,x>674:R,A}
bf{s<3336:A,a<1863:A,R}
glx{x>2156:R,nq}
rxv{s<984:A,R}
plp{s>2690:A,a>3040:R,x<1194:A,R}
zdd{x<411:A,s<2583:A,s<3374:R,R}
sbp{x>2442:R,m>802:A,a<1990:A,A}
rzc{m>1519:R,a>2789:A,R}
hgj{x>992:kbh,a>2782:hql,mdb}
gc{m>3015:A,a<3333:A,x<1181:R,A}
xh{a>3215:A,x<3863:qtm,lnj}
jdx{a>296:hz,s>2019:A,a<185:R,R}
ftz{a<2615:A,A}
sr{a>3503:A,m<3116:R,vt}
bz{m<1363:A,a<3918:R,x>347:A,A}
lxl{m>375:ts,x>2633:ssr,zz}
rpl{a<3086:zg,km}
rcc{a<3086:lx,x<186:A,x<206:jjf,bkt}
xjz{s>2664:dkl,m>1134:rzc,a<2824:klr,R}
kf{m>2364:jp,clb}
chs{s>2600:qd,R}
td{a>2469:A,a>2389:A,x>480:A,R}
st{m<3511:R,x<3456:R,s>3602:cvj,rpd}
hmq{a>2111:A,x>1851:R,A}
xc{x<505:zr,pl}
th{x<582:pm,lk}
sn{m<2549:R,s>2203:R,dlr}
chx{m>3054:A,a<1763:R,A}
kk{m>3497:A,a<1434:A,R}
xj{a>2832:lkb,s<3601:A,s>3811:rbp,rfg}
ncd{a<2175:R,R}
dmv{x>1035:A,m<796:R,x>598:jl,bz}
zdz{s>3645:R,x>2044:R,a<2560:R,xv}
bj{x>1208:R,x<1157:R,m>1150:A,R}
sbr{m<3368:R,R}
kx{x<3147:vdh,jvl}
kv{m>3555:bmz,x>265:sl,A}
qd{x>1553:A,a>2834:A,s<3385:A,R}
cxs{s>1563:cxt,x>2102:A,bs}
lt{s<1846:A,x<3520:R,x>3795:R,nkk}
sv{m>3685:R,a>2243:R,R}
ft{a<963:rxt,x>2779:A,x<2244:A,lzv}
rl{a<1094:rr,s>2736:bbb,s>1976:rhb,fr}
jn{x<583:ttn,s<1225:A,A}
jxl{a<2453:rxj,a>2766:ngl,zfp}
cvs{m>285:R,A}
jqh{a<2775:dvm,gbn}
dg{a<2815:A,R}
jvl{x>3689:xh,x<3456:mjp,znq}
gnc{x<140:R,x<171:A,R}
fkh{a<3335:R,A}
vrm{m>937:A,m<448:R,R}
scz{m<2202:qk,gp}
gvq{s<123:A,m<3059:A,a<3441:A,A}
mdb{m>1624:gpj,hf}
xsc{x<1257:bbn,x<1486:A,sxl}
dp{m<1786:A,a<3541:R,A}
sf{m>773:xdr,ffq}
dlr{x<2407:R,s>1899:R,x<2587:R,R}
rk{x>2534:A,A}
rn{s<2830:A,hbb}
jvg{s>3506:A,m>2651:A,m>2621:A,R}
hfg{m>2738:zdz,a>2232:lkv,fvs}
lzv{s<480:A,A}
tqf{m>2290:ssz,xb}
vmf{m<1433:qhx,m>2129:jcz,a<970:xk,dck}
gv{m>563:R,x>1087:bd,m<346:R,vz}
fvs{s>3483:A,a<1973:bf,x>1995:ghn,hmq}
tqk{m<963:R,a>2859:R,A}
rx{x<977:nnk,bqr}
rvm{x>3305:A,m<1917:R,R}
clb{x>1123:R,chh}
pm{s>967:A,vbd}
zcz{a<501:R,R}
fsz{a>2745:R,R}
nfq{s<1487:jbm,a>1745:jjb,rl}
hdh{m>3509:A,s<1411:R,m<3227:A,A}
dgv{m>2115:A,A}
ccf{x>582:sxr,s>2966:qr,s<2421:btt,vmf}
kbh{x<1100:hgk,m>2034:smc,bj}
dd{m<2080:ct,s>2201:rk,hv}
ff{s>381:R,x<841:R,R}
gtx{s<890:A,m<3503:A,x>2000:A,R}
bx{m<3245:R,m>3522:zfz,x<2768:tc,R}
cbz{s<800:kpf,vr}
hjj{a<1141:R,s<3290:R,A}
nnk{s<496:vls,a<1328:nt,m<2251:R,gtr}
mhr{m>1624:R,m<1228:R,m>1458:ggp,mgb}
grb{s>2354:A,s>824:R,s<393:R,A}
vzz{m>2291:R,m<1985:A,A}
dc{s<2446:A,s>2870:A,s>2640:A,R}
cb{s>956:kb,A}
bkv{s<917:bm,jn}
cmn{a<1575:A,s>2325:A,s<2132:pzh,A}
vq{x>1200:R,m<1709:A,A}
xx{s<2987:R,s>3641:A,m<3099:gnk,A}
skj{x<2732:pgx,x>2863:pqn,a>2109:R,A}
qk{x<433:R,x<540:grb,s>2612:dp,mf}
nb{a<3379:dgv,x>433:xqb,mzt}
skd{s>85:R,a>3825:R,s>57:bsr,vj}
bsr{a<3615:R,A}
bqr{m<2638:A,kk}
gmm{s<271:R,R}
ssr{m>170:mkm,A}
xr{x>1412:R,R}
dpd{s>2570:rz,a<2463:ndl,m<1077:tqk,fsz}
cg{x>2240:R,s>3092:A,s<2882:R,A}
zt{s>605:R,m<1978:R,x<3468:A,A}
gbn{a>2840:A,x>606:R,A}
sk{s>627:A,R}
xv{s<3310:A,x<1827:R,R}
jvb{m>2641:rhf,x<598:dsq,x>615:R,nr}
dbk{a<3270:R,s<3854:A,R}
ld{x>1033:A,s<1042:A,A}
hz{s>2156:A,R}
hgm{a>3479:zsv,a>3293:jsd,bg}
sd{s<1388:R,m<220:A,a<3336:A,R}
ks{x<3304:R,A}
cp{x<3510:A,s>2786:A,R}
mc{x<138:tx,m<1731:lg,nd}
grf{a<3910:dt,zzd}
tm{m>2653:A,R}
znm{x>3272:R,tj}
qtm{a<2683:A,m<3283:R,A}
cpj{x>523:A,m>2682:A,m<2527:R,A}
vx{x>2368:A,A}
kp{x>444:R,x<233:R,A}
vlq{a>3107:tm,s>3354:A,m<2037:R,R}
mzn{m>2863:R,A}
zb{s>1752:R,x>1070:zgf,sk}
pqn{m<2758:A,R}
lsq{m<3507:A,s<957:R,R}
krr{a<3740:R,R}
czn{a>3163:R,m<1999:R,a<3012:A,R}
nkk{s<2070:R,R}
nrt{s<1315:A,x<2219:R,A}
ds{a<1180:hjj,m<1480:hxc,pf}
xb{x<928:R,s<680:dpt,ngq}
mj{a<2118:R,R}
kpv{x>329:A,A}
dpt{x<1220:R,R}
vls{a<1332:A,a>1469:A,R}
cs{x>1439:R,s>709:A,R}
tl{a>1243:jsj,ds}
xbt{x>1513:A,fz}
hpt{m<2917:R,m<3336:R,R}
fr{m<1904:hdc,xdt}
dsl{x<891:R,a<3061:A,A}
ph{m>522:A,A}
tck{a>3652:zdg,a<3466:vvc,m<1552:gl,pv}
lk{x>628:A,x<605:R,x>614:R,A}
nj{s>2734:A,R}
khd{a>2723:hm,x<1375:lxg,a<2580:A,zx}
vvc{x<813:nb,s<1759:bbv,a<3401:kf,sg}
fh{m<3059:R,sv}
rt{m<2548:ccf,bqs}
dtv{s>2651:njc,kpn}
db{s>1037:R,s>793:A,x<3290:A,A}
rfg{x>2488:R,A}
rxj{a>2073:fxv,A}
zgf{a<3590:R,R}
lxg{a<2574:R,m>3113:A,a>2644:R,A}
pp{x<1117:zn,a<3445:A,R}
hk{a<3058:A,R}
cps{x>673:gv,a<3514:lf,dtv}
bl{m<1269:A,s>3432:A,s>3277:A,cml}
rhb{m<2194:sf,zjt}
rth{s<2408:R,m>1515:A,R}
vs{x>2539:A,A}
znq{m>3125:R,A}
hzc{m>1851:mv,x>716:qg,jx}
tzb{m>3238:tg,xxd}
jxd{a<2457:A,A}
cn{s<3324:A,A}
qqc{x<2085:R,m<164:A,m>259:A,R}
rbp{m>2847:A,m<2787:A,R}
jjj{x>876:A,a<3595:A,R}
smc{s<1533:R,a>2843:plp,x<1198:R,pms}
ckf{s>1717:A,s<1570:R,R}
kl{a<2275:A,A}
fl{s>2371:tzb,vhl}
rz{a<2450:R,s>3365:A,R}
rjv{s<1810:A,m>758:A,s>2893:R,R}
hx{x<671:A,a>2389:R,x>691:R,R}
jp{s<2734:gc,s>3506:A,R}
bnt{m>837:A,m>433:R,m>254:R,R}
hgk{a<2704:A,a>3062:A,nxg}
txn{m>2615:ktj,tv}
pgx{a>1994:R,x>2611:R,A}
xxd{m>2723:A,s<2919:zvg,m>2567:R,vm}
gtr{s>886:A,a<1457:R,R}
jcz{s>2754:R,a>1065:R,a<523:A,A}
chq{x<3072:A,x<3293:R,R}
clz{s<3150:fkd,s>3490:rfx,hgm}
hck{x>2194:R,s>122:A,A}
vd{m>1534:jjs,a>3770:kmt,cj}
qzn{x>1103:R,s<2513:A,A}
pj{m>3377:A,A}
tc{a>3418:R,R}
kt{a<1141:tqf,s>1093:hzc,a<1584:rx,dx}
gf{a<3431:A,A}
jx{s<1412:vrm,s>1565:A,dm}
vrl{x>386:xz,R}
mz{a<2492:A,A}
mkg{s>1805:vc,a>3085:bkv,a<2882:dnt,th}
ns{a>3913:R,bdr}
kq{s<2373:A,bt}
jcs{s>2567:R,s<2521:A,R}
jbm{a<2334:cbz,m<2533:cr,s<516:rpl,kx}
pjm{m<2956:A,pcn}
nt{a>1236:A,m<2027:A,R}
rbf{s<846:R,m>2754:hdh,x<73:qfz,jh}
hf{s<2598:cvv,A}
cl{s>3154:hk,a<2910:dc,m>1368:czn,ccj}
rh{s>2442:jxd,s<2047:ckf,s>2249:df,A}
vj{s<31:R,a<3675:R,a<3727:R,A}
qhx{a<1403:R,s<2780:A,R}
hsb{s<3410:R,a<1336:A,A}
vfc{x<2652:kh,s>1086:bjc,m>636:R,A}
cd{s<1248:R,a>2984:R,A}
nhf{a<3351:bk,gvq}
jmz{x>176:R,a>2694:bnt,a>2565:R,vp}
cfl{a>1330:sbp,s<1255:mkx,m<858:R,pqm}
lkb{a<2993:R,m<2845:R,A}
jjb{a>3094:clz,m<2393:sxj,x<3056:gvv,fl}
rrd{a<2231:A,m>3181:kl,x>325:R,dkj}
fj{m>3651:R,gtx}
hdc{x>2461:znm,s>1764:bfq,s>1606:zgx,cxs}
kmt{x<1020:xjj,s<2087:xr,A}
cj{m<730:txd,x>819:A,kp}
xqb{s<1587:rb,a<3436:R,vpg}
xq{a<3606:R,x<329:R,R}
tq{s<2112:A,s<3362:R,A}
zzp{x>2975:sqz,m<2520:R,R}
pqm{m<1122:A,s<1390:R,a>875:R,A}
jq{x<3451:R,cf}
xjj{s>2372:R,x<668:A,s>1023:R,R}
tfq{s>3172:R,m>1532:R,A}
bqs{x>919:xsc,vrl}
thp{a<2602:R,m<2928:R,x<247:A,A}
dkj{x>141:A,m<2971:R,A}
pzx{x>2642:A,a<3478:A,R}
ths{x<2809:sn,a>725:qvl,m<1362:jdx,lnn}
qg{a>1868:mj,m>836:A,zxc}
tx{s>1835:tph,m<2098:px,rbf}
vf{a<2428:skj,m<2736:pg,x>2633:shv,xj}
km{s>209:zqs,a>3504:skd,x<2757:glx,nhf}
kj{s<3287:nj,thp}
cml{m<1837:A,s<3096:R,a>494:A,R}
rlc{m>474:R,m>251:A,x>2484:A,A}
txd{s>2156:R,A}
gl{a>3563:hhp,cps}
ccj{m>569:A,s<2508:A,R}
pf{m>3126:A,x>3194:R,R}
pbh{m>3239:mm,cpj}
zjt{a>1394:cmn,lv}
drm{a>1592:A,R}
bdr{a<3869:R,m>2643:A,x>1082:A,A}
jqs{s>2614:rlc,dq}
mv{x<623:R,qlj}
nr{m<2107:R,R}
zdg{a<3847:vd,s>2594:pck,ln}
xk{s<2671:A,m>1780:R,A}
px{s<685:zzq,s<1444:A,a<2772:R,A}
znr{m>2244:R,a>2352:A,x<3509:A,R}
bk{s>79:R,x<3271:R,A}
ktj{x<1421:khd,m<3145:xbt,x>1493:chs,zj}
hb{s>3741:R,s>3440:A,gm}
ffs{s<1003:A,m>2845:cd,x>2032:nrt,hdf}
dl{x<3044:R,m>1590:A,m<588:A,A}
vg{m<1260:A,s<201:A,a>302:A,A}
zfp{s>3617:chr,R}
qpt{x<2038:R,s<1806:R,x>2275:R,R}
jrp{m<1617:krr,s<3809:A,A}
zqs{s>394:pzx,tn}
knq{x>908:R,x<872:R,dsl}
bbb{x<2800:xrs,a>1484:jpt,x>3526:vrp,tl}
fz{s<1941:R,R}
nm{a>2984:A,m>3291:A,A}
rrl{s>2543:A,a>1195:A,a>1143:R,R}
jpt{a>1614:zf,m<2537:jq,hlc}
mb{s>2480:A,a>2039:R,A}
flg{x<2119:bth,m<3180:mz,a>2238:fhg,kq}
qlg{a>3593:R,A}
nd{s<1985:jd,a<2754:kj,x<224:rcc,bn}
nsg{x>3601:R,R}
pg{a>2794:A,m>2531:A,tk}
vpg{a<3448:A,x>601:A,m>2200:A,R}
qq{a<3599:qlg,xq}
mjq{m>906:A,R}
tk{s<3534:R,A}
tf{x<2430:A,m<2307:A,x<2765:R,A}
bd{m>361:R,a>3529:A,R}
xsq{a>2809:vl,a<2624:tfq,m<1412:mjq,nhl}
bs{m<1209:R,x<1880:A,A}
fn{a<3588:kv,a<3614:qq,s<1821:cls,rn}
xt{m<2535:A,drm}
pzh{x>3042:R,x>2454:A,R}
sx{x<1429:bvs,rxv}
vz{x<933:R,A}
nq{m<3063:R,A}
dm{m>867:R,A}
sdr{s>1536:zsf,cnc}
bvs{x<1372:A,x<1409:R,m>1611:A,R}
zxk{m>1166:tb,m>571:fkh,sd}
nxg{x>1061:R,x<1037:R,x>1045:R,R}
gq{x<350:mc,a<2674:dz,mkg}
vrp{x<3760:bmc,ddj}
bbn{x>1096:A,m<3379:A,s>2981:R,A}
sc{s>2125:R,A}
vlc{s<330:R,x>2068:R,R}
pl{s>2521:R,x<590:R,R}
shm{m<1775:R,a<392:R,khx}
zvg{s>2600:A,R}

{x=864,m=2222,a=195,s=384}
{x=901,m=448,a=737,s=1768}
{x=2926,m=108,a=66,s=710}
{x=447,m=2075,a=739,s=2}
{x=597,m=681,a=975,s=28}
{x=2159,m=1819,a=2019,s=618}
{x=384,m=15,a=1695,s=142}
{x=76,m=739,a=729,s=42}
{x=1070,m=3064,a=2917,s=2856}
{x=1584,m=1118,a=1134,s=1906}
{x=23,m=7,a=309,s=260}
{x=987,m=52,a=513,s=2353}
{x=2128,m=133,a=1707,s=2272}
{x=1721,m=9,a=10,s=2757}
{x=56,m=1928,a=2447,s=651}
{x=358,m=1514,a=208,s=2003}
{x=991,m=160,a=1850,s=29}
{x=135,m=900,a=257,s=1885}
{x=1594,m=1262,a=292,s=773}
{x=958,m=762,a=1835,s=129}
{x=204,m=851,a=781,s=51}
{x=393,m=77,a=65,s=1317}
{x=102,m=2858,a=694,s=133}
{x=1558,m=357,a=508,s=289}
{x=185,m=111,a=305,s=2389}
{x=877,m=2013,a=2980,s=2509}
{x=2494,m=1224,a=54,s=939}
{x=336,m=1421,a=2434,s=2183}
{x=375,m=713,a=2313,s=459}
{x=2899,m=54,a=334,s=1951}
{x=452,m=585,a=16,s=531}
{x=220,m=566,a=192,s=346}
{x=1556,m=1173,a=7,s=611}
{x=793,m=1179,a=549,s=16}
{x=2730,m=545,a=888,s=2861}
{x=8,m=136,a=1607,s=40}
{x=641,m=2099,a=2295,s=745}
{x=2342,m=100,a=1010,s=1306}
{x=3432,m=432,a=658,s=645}
{x=121,m=1563,a=1172,s=1807}
{x=60,m=1124,a=859,s=595}
{x=39,m=2653,a=1212,s=295}
{x=1810,m=2143,a=250,s=899}
{x=1049,m=1680,a=2472,s=66}
{x=1305,m=645,a=1311,s=18}
{x=32,m=1575,a=933,s=88}
{x=2066,m=2204,a=2147,s=48}
{x=806,m=263,a=862,s=406}
{x=1218,m=142,a=2380,s=2080}
{x=199,m=1608,a=815,s=504}
{x=1355,m=844,a=312,s=186}
{x=409,m=1271,a=2737,s=620}
{x=652,m=409,a=315,s=1051}
{x=2044,m=149,a=2045,s=3394}
{x=1787,m=496,a=142,s=313}
{x=70,m=78,a=953,s=872}
{x=1072,m=1733,a=541,s=115}
{x=254,m=302,a=1483,s=1537}
{x=239,m=1242,a=929,s=15}
{x=517,m=932,a=1899,s=844}
{x=2926,m=364,a=595,s=1294}
{x=1358,m=68,a=34,s=1728}
{x=2065,m=312,a=3623,s=181}
{x=414,m=451,a=876,s=80}
{x=1689,m=843,a=139,s=245}
{x=1136,m=289,a=2256,s=2537}
{x=99,m=1094,a=980,s=382}
{x=1610,m=751,a=456,s=143}
{x=717,m=113,a=530,s=502}
{x=2050,m=1025,a=431,s=48}
{x=3411,m=87,a=306,s=2865}
{x=1894,m=19,a=46,s=3399}
{x=574,m=1848,a=599,s=1924}
{x=3,m=1021,a=26,s=1166}
{x=776,m=1882,a=1946,s=453}
{x=1170,m=381,a=957,s=601}
{x=1703,m=200,a=298,s=1246}
{x=117,m=43,a=286,s=1131}
{x=2642,m=18,a=552,s=8}
{x=333,m=458,a=518,s=1260}
{x=2391,m=794,a=1032,s=1156}
{x=1439,m=629,a=1737,s=268}
{x=70,m=465,a=603,s=1399}
{x=312,m=648,a=875,s=785}
{x=3106,m=348,a=45,s=198}
{x=2991,m=18,a=1424,s=436}
{x=397,m=1490,a=520,s=53}
{x=553,m=232,a=2916,s=781}
{x=1404,m=738,a=925,s=513}
{x=757,m=619,a=442,s=43}
{x=1747,m=924,a=11,s=430}
{x=3644,m=201,a=693,s=33}
{x=336,m=141,a=1473,s=2008}
{x=1723,m=1595,a=91,s=282}
{x=768,m=426,a=895,s=573}
{x=312,m=604,a=12,s=361}
{x=2900,m=23,a=748,s=578}
{x=1100,m=116,a=2804,s=2407}
{x=121,m=94,a=937,s=514}
{x=61,m=2592,a=1518,s=855}
{x=356,m=64,a=1167,s=732}
{x=1538,m=702,a=444,s=524}
{x=1071,m=230,a=1273,s=954}
{x=1538,m=1389,a=963,s=681}
{x=3560,m=407,a=606,s=1015}
{x=276,m=534,a=14,s=2920}
{x=122,m=1529,a=3159,s=302}
{x=764,m=529,a=1535,s=1743}
{x=1015,m=491,a=1323,s=1241}
{x=831,m=1667,a=2553,s=146}
{x=841,m=203,a=2621,s=696}
{x=1586,m=403,a=1308,s=1892}
{x=2119,m=47,a=446,s=124}
{x=2321,m=2184,a=309,s=735}
{x=44,m=99,a=2037,s=1730}
{x=917,m=192,a=31,s=1104}
{x=1135,m=2330,a=2073,s=1651}
{x=1149,m=907,a=51,s=79}
{x=299,m=330,a=2588,s=1829}
{x=203,m=1847,a=205,s=696}
{x=418,m=498,a=1030,s=307}
{x=1560,m=1398,a=54,s=723}
{x=1232,m=39,a=216,s=648}
{x=1449,m=527,a=2553,s=39}
{x=2579,m=154,a=208,s=1099}
{x=213,m=1557,a=2636,s=202}
{x=1787,m=1475,a=1407,s=1761}
{x=1057,m=33,a=29,s=1488}
{x=46,m=2348,a=2573,s=1457}
{x=2080,m=352,a=247,s=97}
{x=166,m=10,a=938,s=1203}
{x=375,m=3545,a=21,s=2507}
{x=1525,m=1748,a=63,s=511}
{x=4,m=712,a=1182,s=2085}
{x=460,m=996,a=1,s=2354}
{x=538,m=1452,a=1066,s=2980}
{x=328,m=488,a=2291,s=1542}
{x=787,m=2124,a=649,s=281}
{x=890,m=17,a=1812,s=99}
{x=950,m=308,a=67,s=87}
{x=792,m=500,a=64,s=546}
{x=194,m=524,a=849,s=1060}
{x=24,m=492,a=1727,s=1628}
{x=3080,m=427,a=11,s=300}
{x=910,m=2320,a=2178,s=225}
{x=100,m=586,a=2247,s=1402}
{x=1277,m=400,a=1053,s=11}
{x=442,m=450,a=1063,s=1664}
{x=1360,m=658,a=1705,s=306}
{x=60,m=1215,a=1012,s=2971}
{x=37,m=973,a=1351,s=2900}
{x=755,m=1798,a=41,s=1999}
{x=456,m=565,a=13,s=18}
{x=1068,m=108,a=1261,s=216}
{x=179,m=559,a=1871,s=1357}
{x=760,m=1634,a=1753,s=502}
{x=33,m=466,a=1114,s=112}
{x=443,m=1297,a=280,s=1777}
{x=2608,m=1971,a=651,s=2930}
{x=244,m=1227,a=1578,s=15}
{x=1063,m=635,a=2276,s=331}
{x=321,m=1326,a=2474,s=359}
{x=1412,m=186,a=2378,s=750}
{x=1007,m=463,a=211,s=520}
{x=2193,m=158,a=1966,s=952}
{x=1047,m=348,a=37,s=88}
{x=3034,m=1692,a=248,s=1957}
{x=508,m=1601,a=213,s=712}
{x=640,m=386,a=1410,s=2062}
{x=683,m=855,a=503,s=3485}
{x=574,m=28,a=978,s=287}
{x=4,m=1189,a=20,s=330}
{x=1716,m=1734,a=166,s=547}
{x=903,m=1023,a=220,s=2002}
{x=2105,m=2151,a=753,s=216}
{x=357,m=1260,a=242,s=32}
{x=382,m=1261,a=3644,s=2198}
{x=451,m=1290,a=1901,s=446}
{x=510,m=391,a=1064,s=1313}
{x=1279,m=40,a=1825,s=946}
{x=21,m=1879,a=1728,s=429}
{x=50,m=183,a=1799,s=895}
{x=2200,m=2092,a=827,s=1450}
{x=230,m=1800,a=1178,s=1276}
{x=747,m=1987,a=838,s=1589}
{x=2113,m=613,a=727,s=26}
{x=34,m=305,a=1750,s=1402}
{x=394,m=3102,a=229,s=339}
{x=247,m=1734,a=102,s=847}
{x=819,m=1651,a=3293,s=1663}
{x=459,m=2156,a=175,s=296}
{x=1194,m=2195,a=324,s=297}
{x=510,m=47,a=9,s=1406}
{x=478,m=18,a=114,s=643}
{x=278,m=2041,a=185,s=237}
{x=2208,m=263,a=252,s=70}
{x=22,m=956,a=1030,s=240}
{x=164,m=466,a=3598,s=1158}
{x=2901,m=267,a=108,s=455}
{x=569,m=1421,a=1176,s=1237}
";
const TEST: &'static str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cond<'a> {
    Less(&'a str, u64, Action<'a>),
    Greater(&'a str, u64, Action<'a>),
    Default(Action<'a>),
}

fn parse_action<'a>(action: &'a str) -> Action<'a> {
    match action {
        "A" => Action::Accept,
        "R" => Action::Reject,
        other => Action::Workflow(other),
    }
}
fn parse<'a>(input: &'a str) -> (HashMap<&'a str, Vec<Cond<'a>>>, Vec<HashMap<&'a str, u64>>) {
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let mut rule_hm = HashMap::new();
    for (workflow, rules) in rules
        .lines()
        .map(|rule| rule.split_once('{'))
        .map(Option::unwrap)
    {
        let rules = &rules[0..rules.len() - 1];
        let mut rules_vec = vec![];
        for rule in rules.split(',') {
            let split_point = rule.find('<').or_else(|| rule.find('>'));
            let cond = if let Some(idx) = split_point {
                let (num, action) = rule[idx + 1..].split_once(":").unwrap();
                let num = num.parse::<u64>().unwrap();
                let action = parse_action(action);
                match &rule[idx..idx + 1] {
                    ">" => Cond::Greater(&rule[0..idx], num, action),
                    "<" => Cond::Less(&rule[0..idx], num, action),
                    _ => unreachable!(),
                }
            } else {
                Cond::Default(parse_action(rule))
            };
            rules_vec.push(cond);
        }
        rule_hm.insert(workflow, rules_vec);
    }

    let mut parts_list = vec![];
    for part in parts.lines() {
        let part = &part[1..part.len() - 1];
        parts_list.push(
            part.split(',')
                .map(|l| l.split_once('=').unwrap())
                .map(|(key, val)| (key, val.parse::<u64>().unwrap()))
                .collect(),
        );
    }

    (rule_hm, parts_list)
}

fn simulate<'a>(part: &HashMap<&'a str, u64>, rules: &HashMap<&'a str, Vec<Cond<'a>>>) -> bool {
    let mut cur = "in";
    'outer: loop {
        for rule in &rules[cur] {
            let action = match rule {
                Cond::Less(key, num, action) if part[key] < *num => Some(action),
                Cond::Greater(key, num, action) if part[key] > *num => Some(action),
                Cond::Default(action) => Some(action),
                _ => None,
            };
            match action {
                Some(Action::Accept) => return true,
                Some(Action::Reject) => return false,
                Some(Action::Workflow(new)) => {
                    cur = new;
                    continue 'outer;
                }
                None => continue,
            }
        }
    }
}
fn combinations(part: &HashMap<&str, (u64, u64)>) -> u64 {
    let combinations = part
        .values()
        .map(|(start, end)| end - start + 1)
        .product::<u64>();
    return combinations;
}

fn simulate2<'a>(
    part: &HashMap<&'a str, (u64, u64)>,
    rules: &HashMap<&'a str, Vec<Cond<'a>>>,
    cur_rule: &str,
    cur_rule_idx: usize,
) -> u64 {
    if cur_rule_idx >= rules[cur_rule].len() {
        return 0;
    }
    match rules[&cur_rule][cur_rule_idx] {
        Cond::Default(Action::Accept) => combinations(part),
        Cond::Default(Action::Reject) => {
            return 0;
        }
        Cond::Default(Action::Workflow(new_rule)) => simulate2(part, rules, new_rule, 0),
        Cond::Less(name, num, action) => {
            // Condition was false for entire range
            if part[name].0 >= num {
                //let mut new_part = part.clone();
                //new_part.entry(name).and_modify(|range| range.0 = num);
                simulate2(&part, &rules, cur_rule, cur_rule_idx + 1)
            } else {
                // If our range is (1..=4000) and condition is prop < 2000, then one range will be (1..=1999) which is true
                // and other will be (2000..=4000) which is false
                let mut true_part = part.clone();
                true_part
                    .entry(name)
                    .and_modify(|range| range.1 = min(range.1, num - 1));
                let true_combinations = match action {
                    Action::Accept => combinations(&true_part),
                    Action::Reject => 0,
                    Action::Workflow(workflow) => simulate2(&true_part, &rules, workflow, 0),
                };
                let mut false_part = part.clone();
                let range = false_part
                    .entry(name)
                    .and_modify(|range| {
                        *range = if range.1 >= num {
                            (num, range.1)
                        } else {
                            (0, 0)
                        }
                    })
                    .or_insert((0, 0));
                let false_combinations = if *range == (0, 0) {
                    0
                } else {
                    simulate2(&false_part, &rules, cur_rule, cur_rule_idx + 1)
                };
                true_combinations + false_combinations
            }
        }
        Cond::Greater(name, num, action) => {
            // Condition was false for entire range
            if part[name].1 <= num {
                simulate2(&part, &rules, cur_rule, cur_rule_idx + 1)
            } else {
                // If our range is (1..=4000) and condition is prop > 2000, then one range will be (1..=1999) which is false
                // and other will be (2000..=4000) which is true
                let mut true_part = part.clone();
                true_part
                    .entry(name)
                    .and_modify(|range| range.0 = max(range.0, num + 1));
                let true_combinations = match action {
                    Action::Accept => combinations(&true_part),
                    Action::Reject => 0,
                    Action::Workflow(workflow) => simulate2(&true_part, &rules, workflow, 0),
                };
                let mut false_part = part.clone();
                let range = false_part
                    .entry(name)
                    .and_modify(|range| {
                        *range = if range.0 <= num {
                            (range.0, num)
                        } else {
                            (0, 0)
                        }
                    })
                    .or_insert((0, 0));
                let false_combinations = if *range == (0, 0) {
                    0
                } else {
                    simulate2(&false_part, &rules, cur_rule, cur_rule_idx + 1)
                };
                true_combinations + false_combinations
            }
        }
    }
}

fn main() {
    let (rules, parts) = parse(TEST);
    println!(
        "{}",
        parts
            .iter()
            .filter(|part| simulate(part, &rules))
            .map(|part| part.values().sum::<u64>())
            .sum::<u64>()
    );
    let mut part = HashMap::new();
    part.insert("x", (1, 4000));
    part.insert("m", (1, 4000));
    part.insert("a", (1, 4000));
    part.insert("s", (1, 4000));
    println!("Part 2: {}", simulate2(&part, &rules, "in", 0));

    let (rules, parts) = parse(INPUT);
    println!(
        "{}",
        parts
            .iter()
            .filter(|part| simulate(part, &rules))
            .map(|part| part.values().sum::<u64>())
            .sum::<u64>()
    );
    println!("Part 2: {}", simulate2(&part, &rules, "in", 0));
}
