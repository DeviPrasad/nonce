#!/usr/local/bin/python3
import os
import base64
import json
import tornado.ioloop
import tornado.web
from tornado.options import options, define

define("host", default="loiter.xyz.in", help="app host", type=str)
define("port", default=45001, help="app port", type=int)
define("webdir", default="var/www/", help="app webdir", type=str)

class PkceCodeGrantRedirect(tornado.web.RequestHandler):
    def set_default_headers(self):
        self.set_header("Access-Control-Allow-Origin", "*")
        self.set_header("Vary", "Origin")
        self.set_header('Access-Control-Allow-Methods', "GET, POST, OPTIONS")
        self.set_header("Access-Control-Allow-Headers", "x-requested-with,access-control-allow-origin, authorization,content-type")
        print("\n\n")
        print("PkceCodeGrantRedirect::set_default_headers", self.request)
    def options(self):
        self.set_status(204)
        self.finish()
        print("PkceCodeGrantRedirect::options")
    def prepare(self):
        header = "Content-Type"
        body = "application/json"
        self.set_header(header, body)
        print("PkceCodeGrantRedirect::prepare()")
    def initialize(self):
        print("PkceCodeGrantRedirect::initialize()")
    def get(self):
        clid = self.get_argument('client_id')
        state = self.get_argument('state')
        code = self.get_argument('code')
        self.write(json.dumps({
            "code": code,
            "state": state,
            "client_id": clid}))
        print("PkceCodeGrantRedirect::get")
        print(self.request)
        print(self.request.path)
        print(self.request.query_arguments)

class LoiterRequestHandler(tornado.web.RequestHandler):
    def prepare(self):
        header = "Content-Type"
        body = "application/json"
        self.set_header(header, body)
        print("LoiterRequestHandler::prepare()")
    def initialize(self):
        print("LoiterRequestHandler::initialize()")

class LoiterWebLobbyList(LoiterRequestHandler):
    def set_default_headers(self):
        self.set_header("Access-Control-Allow-Origin", "*")
        self.set_header("Vary", "Origin")
        self.set_header('Access-Control-Allow-Methods', "GET, POST, OPTIONS")
        self.set_header("Access-Control-Allow-Headers", "x-requested-with,access-control-allow-origin,authorization,content-type")
        print("LoiterWebLobbyList::set_default_headers")
    def get(self):
        print(self.request)
        print(self.request.path)
        self.redirect("http://oauth.indus.in:40401/authorize?client_id=ASK123QwErTy&state=BEADBEEF", status=303)
        self.add_header("status", 303)
        print("LoiterWebLobbyList::get")
    def options(self):
        self.set_status(204)
        self.finish()
        print("LoiterWebLobbyList::options")
    def initialize(self, **args):
        LoiterRequestHandler.initialize(self)
        self.scope = args['scope']
        print("LoiterWebLobbyList::initialize")
        print(args)

class LoiterWebLobbyReader(LoiterRequestHandler):
    def set_default_headers(self):
        self.set_header("Access-Control-Allow-Origin", "*")
        self.set_header("Vary", "Origin")
        self.set_header('Access-Control-Allow-Methods', "GET, POST, OPTIONS")
        self.set_header("Access-Control-Allow-Headers", "x-requested-with,access-control-allow-origin,authorization,content-type")
        print("LoiterWebLobbyRead::set_default_headers")
    def options(self):
        self.set_status(204)
        self.finish()
        print("LoiterWebLobbyReader::options")
    def get(self, lobby_name):
        print(self.request)
        print(self.request.path)
        #self.write(json.dumps({'scope': self.scope, 'lobby': lobby_name}))
        self.redirect("http://oauth.indus.in:40401/authorize?client_id=1237890", status=200)
    def initialize(self, **args):
        LoiterRequestHandler.initialize(self)
        self.scope = args['scope']
        print("LoiterWebLobbyReader::initialize")
        print(args)


class LoiterWebClient:
    @classmethod
    def create_loiter_web(cls, webdir, host):
        return tornado.web.Application([
            (r"/lobby", LoiterWebLobbyList, dict(scope="loiter.lobby.list")),
            (r"/lobby/(.*)", LoiterWebLobbyReader, dict(scope="loiter.lobby.read")),
            (r"/pkce/code/redirect", PkceCodeGrantRedirect, dict()),
            (r'/(.*)', tornado.web.StaticFileHandler, {'path': webdir}),
        ],
        default_host=host)


def main(web_base_dir, port):
    print(options)
    app = LoiterWebClient.create_loiter_web(web_base_dir, port)
    http_server = tornado.httpserver.HTTPServer(app)
    http_server.listen(port)
    print("Loiter Confidential Client Started")
    tornado.ioloop.IOLoop.instance().start()

if __name__ == "__main__":
    options.parse_command_line()
    main(options.webdir, options.port)
