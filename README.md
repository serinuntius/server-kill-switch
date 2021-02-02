# server-kill-switch
server-kill-switch is able to shutdown server easily with iOS shortcut.


## installation
```bash
cargo install --git https://github.com/serinuntius/server-kill-switch --branch main
```


## how to use
```bash
sudo vi /etc/systemd/system/server-kill-switch.service
```

```
[Unit]
Description = server-kill-switch is able to shutdown server easily with iOS shortcut.

[Service]
ExecStart = <which server-kill-switch> # like /home/user/.cargo/bin/server-kill-switch
Restart = always
Type = simple

[Install]
WantedBy = multi-user.target
```

```bash
sudo systemctl enable server-kill-switch.service
sudo systemctl start server-kill-switch.service
```
