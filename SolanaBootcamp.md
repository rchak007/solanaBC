# Solana Bootcamp Q2 2024





## Lesson 1 - Monday - 6/3/24



![image-20240603083554124](./Images/image-20240603083554124.png)





![image-20240603083623812](./Images/image-20240603083623812.png)



![image-20240603083707308](./Images/image-20240603083707308.png)





![image-20240603083803393](./Images/image-20240603083803393.png)





![image-20240603083816997](./Images/image-20240603083816997.png)





![image-20240603083943515](./Images/image-20240603083943515.png)



![image-20240603084011329](./Images/image-20240603084011329.png)





![image-20240603084034556](./Images/image-20240603084034556.png)



![image-20240603084146981](./Images/image-20240603084146981.png)





![image-20240603084240548](./Images/image-20240603084240548.png)





![image-20240603084319557](./Images/image-20240603084319557.png)





anchor framework.

![image-20240603084522892](./Images/image-20240603084522892.png)



sli.do application. During lesson.

Discord Readme channel.





![image-20240603085252122](./Images/image-20240603085252122.png)





![image-20240603085350395](./Images/image-20240603085350395.png)

security auditing and development. But education is big focus.



![image-20240603085447264](./Images/image-20240603085447264.png)





![image-20240603085601455](./Images/image-20240603085601455.png)

![image-20240603085953117](./Images/image-20240603085953117.png)

IMP - put this in your overall knowledge category.



![image-20240603090046258](./Images/image-20240603090046258.png)



![image-20240603090223983](./Images/image-20240603090223983.png)



![image-20240603090335121](./Images/image-20240603090335121.png)



![image-20240603090422410](./Images/image-20240603090422410.png)



![image-20240603090542579](./Images/image-20240603090542579.png)





![image-20240603090631905](./Images/image-20240603090631905.png)

![image-20240603090729199](./Images/image-20240603090729199.png)



![image-20240603090819820](./Images/image-20240603090819820.png)



game theory -use economic incentive.. 



![image-20240603090853107](./Images/image-20240603090853107.png)





![image-20240603090904296](./Images/image-20240603090904296.png)





![image-20240603090935979](./Images/image-20240603090935979.png)







![image-20240603091023678](./Images/image-20240603091023678.png)

Imp = check the slide again what he said about this above pic.. 9:11AM. 

Data - little bit.

![image-20240603091056589](./Images/image-20240603091056589.png)

![image-20240603091246170](./Images/image-20240603091246170.png)

![image-20240603091322638](./Images/image-20240603091322638.png)



Imp - have chatGpt break down - Elliptic curves.

taking a point on curve and then take another point to guess something etc. .

![image-20240603091335524](./Images/image-20240603091335524.png)



diff algo work differently.

https://github.com/ExtropyIO/SolanaBootcamp



![image-20240603091457236](./Images/image-20240603091457236.png)



Slot- timestamps. Epochs.

Link before this - parent blockhash.

Child slot;;

Details of txns.



![image-20240603091557951](./Images/image-20240603091557951.png)



Block size is important factor.

IMP - what's BTC blocksize and WTH?



![image-20240603091808049](./Images/image-20240603091808049.png)



explorers:

![image-20240603091824438](./Images/image-20240603091824438.png)



![image-20240603091843749](./Images/image-20240603091843749.png)



![image-20240603091943042](./Images/image-20240603091943042.png)

TXns dont get picked up right away and are in pending state.

Ex BTC - 10 mins - mempool can get large. 

but ETH and SOL it ets complicated. MEV 

IMP - MEV - 

its public people know what Txns are happenning. Nodes can decide what to use based on that.

Block producers can extract more value. 

higher Txn fee they can pick.

lot of demand - large mempool.

![image-20240603092200199](./Images/image-20240603092200199.png)





Solana used improvements .. around scalability and speed. 

message works within Solana. 

Gossip protocol was unstructured way and unfocused. not geaographic proximity. etc .

mechanisms to improve on that. 

SOL - more targeted.

we know who the block produceer will be - so why dont we just send Txn to that person - then they can put in the block. Then they can pass it around. initially only producer needs it.

So main thing we know who wil be Leader (block producer).

This reduces load on the system. IMP - to make it more scalable etc

![image-20240603092433382](./Images/image-20240603092433382.png)



![image-20240603092618013](./Images/image-20240603092618013.png)







![image-20240603092649571](./Images/image-20240603092649571.png)



Deterministic Leader! IMP



QUestion - 

If we only send the txn to one single blockproducer, wouldn't that introduce a sense of centralisation? what if they don't include our txn?

Erick Fernandez

now

The rotating leader schedule reduces this risk by sending transactions to multiple validators



IMP - listen 925AM.





![image-20240603092918620](./Images/image-20240603092918620.png)



![image-20240603093022018](./Images/image-20240603093022018.png)

9:28 - concurrency.. listen again.



![image-20240603093055851](./Images/image-20240603093055851.png)



Txn 1 , 2 and 4 can be separated things out and run paraellel and then Txn 3.

![image-20240603093155664](./Images/image-20240603093155664.png)



![image-20240603093310642](./Images/image-20240603093310642.png)





IMP - gossip protocol - listen early and also research.



### Proof of History

![image-20240603093353367](./Images/image-20240603093353367.png)





There is no Clock available.

Block producer sequences Txns.

### Consensus 2 parts

![image-20240603093545489](./Images/image-20240603093545489.png)





![image-20240603093653253](./Images/image-20240603093653253.png)



![image-20240603093751801](./Images/image-20240603093751801.png)



![image-20240603093803183](./Images/image-20240603093803183.png)



random function used by some blockchains.

### liveness

if no producer is chosen? 

in BTC eventually someone will solve the puzzle.

If you assign slot - what is person chosen if offline or cant produce one? 
Then how do we proceed. part of the solution could be based on time slots.

Then next slot can produce the block.

So we need reliable source of time.

![image-20240603094052587](./Images/image-20240603094052587.png)



BFT is general name. 

Nakamotu consensus. 

![image-20240603094256889](./Images/image-20240603094256889.png)



crypto economic incentive to behave.



![image-20240603094407760](./Images/image-20240603094407760.png)



![image-20240603094520426](./Images/image-20240603094520426.png)



![image-20240603094541515](./Images/image-20240603094541515.png)

voting takes place among certain nodes - they dont scale very well. Since once you have lot of nodes it takes time. 

If some nodes dont vote.. some faults system has to proceed



Lesson 2 - 

![image-20240603094746605](./Images/image-20240603094746605.png)



![image-20240603094824804](./Images/image-20240603094824804.png)



there is no centralized auth for time.

![image-20240603094937059](./Images/image-20240603094937059.png)



hash - commit scheme that are binding. one way functions.

![image-20240603095126334](./Images/image-20240603095126334.png)



![image-20240603095201029](./Images/image-20240603095201029.png)



![image-20240603095302921](./Images/image-20240603095302921.png)



this has to be done sequentailly but we cna verify in parallel.





![image-20240603095429822](./Images/image-20240603095429822.png)



![image-20240603095558565](./Images/image-20240603095558565.png)





### homework

![image-20240603095617271](./Images/image-20240603095617271.png)

![image-20240603095638918](./Images/image-20240603095638918.png)





![image-20240603095658916](./Images/image-20240603095658916.png)



![image-20240603095724322](./Images/image-20240603095724322.png)











Question - 

Anonymous

15 minutes ago

Could you share a link to some resource on how solana manages to sync everything so quickly?

Erick Fernandez

13 minutes ago

Here's the Solana whitepaper: https://github.com/solana-labs/whitepaper/blob/master/solana-whitepaper-en.pdf



























