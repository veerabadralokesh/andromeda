import valkey
# r = valkey.Valkey(host='localhost', port=6379, db=0, protocol=3)

# Connect using Pool
pool = valkey.ConnectionPool(host='localhost', port=6379, db=0, protocol=3, max_connections=100)
r = valkey.Valkey(connection_pool=pool)
try:
    print(r.set('foo', 'bar'))
    print(r.set('foo', 'bar'))
    print(r.get('foo'))

    ##


    ## Execute multiple operations in a go
    pipe = r.pipeline()
    pipe.set('foo', 5)
    pipe.set('bar', 18.5)
    pipe.set('blee', "hello world!")
    pipe.set('message', "hello valkey")
    print(pipe.execute())
    # [True, True, True]

    print(r.get('foo'))



    ## pub sub, python client can only subscribe use valkey-cli to publish.
    p = r.pubsub()

    p.subscribe('channel1')

    i = 0
    while i < 100:
        msg = p.get_message(timeout=1)
        if msg is None:
            i += 1
        else:
            i = 0
        print(msg)
finally:
    r.close()
    pool.close()
