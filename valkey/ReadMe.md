# Valkey

### Run Valkey without persistence
```bash
docker run --rm -name valkey -p 6379:6379 -it valkey/valkey:latest
```

### Run Valkey in detached mode without persistence
```bash
docker run -d --rm -name valkey -p 6379:6379 -it valkey/valkey:latest
```

### Run Valkey with persistant storage
```bash
docker run --rm --name valkey -p 6379:6379 -v ~/data/db/valkey:/data -d valkey/valkey valkey-server --save 60 1 --loglevel warning
```

### Run using docker-compose
```bash
docker-compose up -d
```
This will run the valkey till its stopped. To stop run 
```bash
docker-compose down
```

### Python

Use [valkey-py](https://github.com/valkey-io/valkey-py) module to connect to the database.

```bash
pip install valkey[libvalkey]
```

```python
>>> import valkey
>>> r = valkey.Valkey(host='localhost', port=6379, db=0, protocol=3)
>>> r.set('foo', 'bar')
True
>>> r.get('foo')
b'bar'
```

Run the `demo.py` file
```bash
python demo.py
```

