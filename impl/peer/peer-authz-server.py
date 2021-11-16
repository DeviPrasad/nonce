#!/usr/local/bin/python3
import os
import base64
import json
import tornado.ioloop
import tornado.web
from tornado.options import options, define

define("host", default="peer.authn.com", help="app host", type=str)
define("port", default=60000, help="app port", type=int)
define("webdir", default="var/www/", help="app webdir", type=str)

class PeerAuthZRequestHandler(tornado.web.RequestHandler):
    def prepare(self):
        header = "Content-Type"
        body = "application/json"
        self.set_header(header, body)
        print("PeerAuthZRequestHandler::prepare()")
    def initialize(self):
        print("PeerAuthZRequestHandler::initialize()")

class StaticFileHandler(tornado.web.StaticFileHandler):
    def set_extra_headers(self, path):
        self.set_header("Access-Control-Allow-Origin", "*")
        self.set_header("Vary", "Origin")
        self.set_header('Access-Control-Allow-Methods', "GET")
        self.set_header("Access-Control-Allow-Headers", "Access-Control-Allow-Origin")
        print("StaticFileHandler::set_default_headers: ", path)

class PeerAuthzRequestProcessor(PeerAuthZRequestHandler):
    def set_default_headers(self):
        self.set_header("Access-Control-Allow-Origin", "*")
        self.set_header("Vary", "Origin")
        self.set_header('Access-Control-Allow-Methods', "GET")
        self.set_header("Access-Control-Allow-Headers", "Access-Control-Allow-Origin")
        print("PeerAuthzRequestProcessor::set_default_headers")
    def get(self):
        print(self.request)
        print(self.request.path)
        self.redirect("authn.html", status=303)
        print("PeerAuthzRequestProcessor::get")
    def options(self):
        self.set_status(204)
        self.finish()
        print("PeerAuthzRequestProcessor::options")
    def initialize(self):
        PeerAuthZRequestHandler.initialize(self)
        print("PeerAuthzRequestProcessor::initialize")

class PeerAuthZServer:
    @classmethod
    def create_instance(cls, webdir, host):
        return tornado.web.Application([
            (r"/authorize", PeerAuthzRequestProcessor),
            (r"/(.*)", StaticFileHandler, {'path': webdir, 'default_filename': 'authn.html' }),
        ], default_host=host)


def main(web_base_dir, port):
    print(web_base_dir, port)
    app = PeerAuthZServer.create_instance(web_base_dir, port)
    http_server = tornado.httpserver.HTTPServer(app)
    http_server.listen(port)
    print("Peer AuthZ Server Started")
    tornado.ioloop.IOLoop.instance().start()

if __name__ == "__main__":
    options.parse_command_line()
    main(options.webdir, options.port)
