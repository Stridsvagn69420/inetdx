# inetdx

Rust implementation of inetd internal services

| Service  | Port | RFC     | Notes                            |
| -------- | ---- | ------- | -------------------------------- |
| Echo     | 7    | RFC 862 | -                                |
| Discard  | 9    | RRC 863 | -                                |
| Daytime  | 13   | RFC 867 | chrono and RFC 2822              |
| QotD     | 17   | RFC 865 | Custom list or fallack /etc/motd |
| Chargen  | 19   | RFC 864 | DoS concerns                     |
| Time     | 37   | RFC 868 | Unix Timestamp as u64            |
| Hostname | 42   | -       | cat /etc/hostname over TCP+UDP   |
