# Usage

## Basic

Prices helps you keep track of your product, and by default, if invoked without any argument, will just print them in a nice way
```
$ prices                                  
* Default
+ CAT&Co
- catnip            10.00€  10.0%  20.0%   13.20€
- scratching post   20.00€  10.0%  10.0%   24.20€
- food               5.00€  10.0%  30.0%    7.15€
+ BIRDstuff
- seeds              2.00€   4.0%  30.0%    2.70€
+ DOGcentre
- leash              5.00€   6.0%  30.0%    6.89€
- toy                3.00€  20.0%  20.0%    4.32€
- metal bowl         8.00€  10.0%  30.0%   11.44€
- food               6.00€  10.0%  20.0%    7.92€
```
if you have *a lot* of products or suppliers and you just want to see one of them, you can use the falg `-s, --supplier` with the name of the supplier, or the flag `-p, --product` with the name of the product
```
$ prices -p food                          
+ CAT&Co
- food    5.00€  10.0%  30.0%    7.15€
+ DOGcentre
- food    6.00€  10.0%  20.0%    7.92€
```
to add a new supplier, combine the flags `-s` and `-a, --add`; to add a new product add the flag `-p` followed by the price paid, the tax percentage and your gain percentage. The resulting sell price will be automatically calculated
```
$ prices --add -s GoldfishLovers
$ prices --add -s GoldfishLovers -p aquarium 40 20 20
$ prices -s GoldfishLovers                        
+ GoldfishLovers
- aquarium   40.00€  20.0%  20.0%   57.60€
```
removing a supplier (and it's products) is a matter of switching from the add to the `-r, --remove` flag; while for removing a single product, only the name is needed
```
$ prices -s BIRDstuff                                
+ BIRDstuff
- seeds                     2.00€   4.0%  30.0%    2.70€
- I am a mistake example    0.00€   0.0%   0.0%    0.00€
$ prices --remove -s BIRDstuff -p 'I am a mistake example'
$ prices -s BIRDstuff
+ BIRDstuff
- seeds    2.00€   4.0%  30.0%    2.70€
$ prices -r -s BIRDstuff 
$ prices -s BIRDstuff            
```

## Advanced

Instead of your gain, you may want to provide the sell price and to calculate the former, if so provide the flag `-f, --final` when adding a product
```
$ prices --add -s GoldfishLovers -p pump 30 20 40 --final
$ prices -s GoldfishLovers                       
+ GoldfishLovers
- aquarium   40.00€  20.0%  20.0%   57.60€
- pump       30.00€  20.0%  11.1%   40.00€
```
mistakes happens, so the option `-u, --undo` to undo the last change may come in handy
```
$ prices -a -s GoldfishLovers -p 'tennis ball' 2 10 10
$ prices -p 'tennis ball' 2 10 10                      
+ GoldfishLovers
- tennis ball    2.00€  10.0%  10.0%    2.42€
$ prices -u
$ prices -a -s DOGcentre -p 'tennis ball' 2 10 10 
$ prices -p 'tennis ball' 2 10 10                 
+ DOGcentre
- tennis ball    2.00€  10.0%  10.0%    2.42€
```

## Manual edit

To store the data, prices uses the file `~/.local/share/prices/prices.json`, along with a pre-last-edit backup. This mean that in case of emergency (e.g.: prices doesn't work anymore) no data is lost, and everything can be read raw from that file.
Also you may want or need to interact with that json manually, any change will instantly be visible through prices.