import string
import base64
import hashlib
import secrets
import traceback
import requests
import time
from requests.adapters import HTTPAdapter
from requests.packages.urllib3.util.retry import Retry


class Pkce:
    urlsafe_alphabet = string.digits + string.ascii_letters + "-._~"

    def __init__(self, session_adapter):
        self.code_verifier_ascii = None
        self.code_challenge = None
        self.urlsafe_ascii_str = None
        self.session_adapter = session_adapter
        self.hash_bytes = None
    # random URL-safe text string, Base64 encoded.
    # On average, each byte results in approximately 1.3 characters.
    def urlsafe_random_token_str(nb):
        return secrets.token_urlsafe(nb)

    def urlsafe_random_str(nc):
        return ''.join(secrets.choice(Pkce.urlsafe_alphabet) for i in range(nc))

    def code_verifier(self):
        print("create_code_verifier")
        try:
            self.code_verifier = None
            self.code_challenge = None
            self.urlsafe_ascii_str = Pkce.urlsafe_random_str(86)
            self.code_verifier_ascii = self.urlsafe_ascii_str.encode("ascii")
            return self.code_verifier_ascii
        except Exception:
            traceback.print_exc()
    def challenge(self):
        try:
            dig = hashlib.sha256()
            dig.update(self.code_verifier_ascii)
            self.hash_bytes = dig.digest()
            self.code_challenge = base64.urlsafe_b64encode(self.hash_bytes)
        except Exception:
            traceback.print_exc()
    def request_code(self):
        self.code_verifier()
        print("request_code - url_safe_str:", self.urlsafe_ascii_str)
        print("request_code - code_verifier:", self.code_verifier_ascii)
        self.challenge()
        print("request_code - code_challenge:", self.code_challenge)
        print("request_code - code_challenge_hex:", self.hash_bytes.hex().upper())
        t0 = time.time()
        try:
            session = self.session_adapter.instance()
            session.get(self.session_adapter.authorization_endpoint(),
            timeout=self.session_adapter.timeout_seconds())
        except requests.exceptions.HTTPError as errh:
            print(errh)
        except requests.exceptions.ConnectionError as errc:
            print(errc)
        except requests.exceptions.Timeout as errt:
            print(errt)
        except requests.exceptions.RequestException as err:
            print(err)
        finally:
            t1 = time.time()
            print('Took', t1 - t0, 'seconds')

class HttpSessionAdapter:
    def authorization_endpoint(self):
        return "http://indus.auth.in:40401/authorize"
    def timeout_seconds(self):
        return 6
    def retry_count(self):
        return 0
    def backoff_factor(self):
        return 0.1
    def status_forcelist(self):
        return (500, 502, 504)
    def instance(self):
        session = requests.Session()
        retry = Retry(total=self.retry_count(), read=self.retry_count(), connect=self.retry_count(),
                        backoff_factor=self.backoff_factor(),
                        status_forcelist=self.status_forcelist(),)
        adapter = HTTPAdapter(max_retries=retry)
        session.mount('http://', adapter)
        session.mount('https://', adapter)
        return session

if __name__ == '__main__':
    pk = Pkce(HttpSessionAdapter())
    pk.request_code()
