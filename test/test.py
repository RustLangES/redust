from redust import Redust
import unittest
import time

redust = None

class TestRedust(unittest.TestCase):
    def test_auth(self):
        self.assertEqual(redust.auth('password'), "OK")

    def test_basics(self):
        self.assertEqual(redust.set('foo', 'bar'), "OK")
        self.assertEqual(redust.get('foo'), "bar")
        self.assertEqual(redust.increment('foo', 1), "Invalid type")

        self.assertEqual(redust.set('foo', 1), "OK")
        self.assertEqual(redust.increment('foo', 1), "OK")
        self.assertEqual(redust.get('foo'), "2")
        self.assertEqual(redust.decrement('foo', 1), "OK")
        self.assertEqual(redust.get('foo'), "1")

        self.assertEqual(redust.rename('foo', 'bar'), "OK")
        self.assertEqual(redust.get('foo'), "Not found")
        self.assertEqual(redust.get('bar'), "1")

        self.assertEqual(redust.copy('bar', 'foo'), "OK")
        self.assertEqual(redust.get('foo'), "1")

        self.assertEqual(redust.delete('foo', 'bar'), "OK")
        self.assertEqual(redust.exists('foo', 'bar'), "0")
    
    def test_expire(self):
        self.assertEqual(redust.set('foo', 'bar'), "OK")
        self.assertEqual(redust.expire('foo', 1), "OK")
        self.assertEqual(redust.ttl('foo'), "1")
        self.assertEqual(redust.copy('foo', 'bar'), "OK")
        self.assertEqual(redust.ttl('bar'), "1")
        self.assertEqual(redust.persist('bar'), "OK")
        time.sleep(2)
        self.assertEqual(redust.ttl('foo'), "-2")
        self.assertEqual(redust.exists('foo'), "0")
        self.assertEqual(redust.exists('bar'), "1")


if __name__ == '__main__':
    redust = Redust(('localhost', 6969))
    assert redust.get('bar') == "Not authenticated" 

    unittest.main()
    redust.close()