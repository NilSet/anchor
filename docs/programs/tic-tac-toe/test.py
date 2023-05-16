from solders.pubkey import Pubkey
from py_client.accounts import Game
from anchorpy import Provider
import asyncio

async def main():
    addr = Pubkey.from_string("3Uk6ZN41pRGEpiwRDMoG7tXGUnF33Mda6YxRS1JS6v8p")
    provider = Provider.local()
    acc = await Game.fetch(provider.connection, addr)
    obj = acc.to_json()
    print(obj)

asyncio.run(main())
