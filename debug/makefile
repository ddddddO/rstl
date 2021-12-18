build_tel_srv:
	docker build -t telnet_server -f dockerfile .

run_tel_srv:
	docker run -p 1123:23 --privileged -d telnet_server /sbin/init

# NOTE: docker exec -it $(CONTAINER_ID) bash でログインして、以下を実行する。
# systemctl start telnet.socket

# ↓ localhostから、tel_srvコンテナへtelnetで接続
# 18:46:15 > telnet localhost 1123
# Trying 127.0.0.1...
# Connected to localhost.
# Escape character is '^]'.
#
# Kernel 4.19.84-microsoft-standard on an x86_64
# 266273320ae7 login: telnet-user
# Password:
# [telnet-user@266273320ae7 ~]$
