from socket import socket, AF_INET, SOCK_STREAM

import json 
import time

class Redust:
    def __init__(self, address) -> None:
        self.sock = socket(AF_INET, SOCK_STREAM)
        self.sock.connect(address)
    
    def close(self: object) -> None:
        self.sock.close()

    def _send_cmd(self: object, cmd: list[str]) -> str:
        self.sock.send("\n".join(cmd).encode() + b';')
        msg = self.sock.recv(8192)
        return msg.decode("utf-8")

    def auth(self: object, password: str) -> str:
        return self._send_cmd(['AUTH', password])
    
    def set(self: object, key: str, value: any) -> str:
        type = 'STRING'

        if isinstance(value, int):
            type = 'INT'
        elif isinstance(value, float):
            type = 'FLOAT'
        elif isinstance(value, bool):
            type = 'BOOL'

        return self._send_cmd(['SET', type, key, str(value)])
    
    def get(self: object, key: str) -> str:
        return self._send_cmd(['GET', key])
    
    def increment(self: object, key: str, amount: float) -> str:
        return self._send_cmd(['INCREMENT', key, str(amount)])
    
    def decrement(self: object, key: str, amount: float) -> str:
        return self._send_cmd(['DECREMENT', key, str(amount)])
    
    def expire(self: object, key: str, seconds: float) -> str:
        return self._send_cmd(['EXPIRE', key, str(seconds)])
    
    def expiretime(self: object, key: str, seconds: float) -> str:
        return self._send_cmd(['EXPIRETIME', key, str(seconds)])
    
    def ttl(self: object, key: str) -> str:
        return self._send_cmd(['TTL', key])

    def rename(self: object, key: str, new_key: str) -> str:
        return self._send_cmd(['RENAME', key, new_key])
    
    def copy(self: object, key: str, new_key: str) -> str:
        return self._send_cmd(['COPY', key, new_key])
    
    def delete(self: object, *keys: str) -> str:
        return self._send_cmd(['DEL', *keys])
    
    def exists(self: object, *keys: str) -> str:
        return self._send_cmd(['EXISTS', *keys])
    
    def persist(self: object, key: str) -> str:
        return self._send_cmd(['PERSIST', key])
