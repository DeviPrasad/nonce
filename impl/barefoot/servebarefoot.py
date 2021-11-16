#!/usr/local/bin/python3
import tornado.ioloop
import tornado.web
from tornado.options import options, define

define("host", default="barefoot.in", help="app host", type=str)
define("port", default=30001, help="app port", type=int)
define("webdir", default="var/www/", help="app webdir", type=str)

class BarefootWeb:
    @classmethod
    def create(cls, webdir, host):
        return tornado.web.Application([
            (r'/(.*)', tornado.web.StaticFileHandler, {'path': webdir}),
        ],
        default_host=host)


def main(web_base_dir, port):
    print(options)
    app = BarefootWeb.create(web_base_dir, port)
    barefoot_server = tornado.httpserver.HTTPServer(app)
    barefoot_server.listen(port)
    print("Loiter Confidential Client Started")
    tornado.ioloop.IOLoop.instance().start()

if __name__ == "__main__":
    options.parse_command_line()
    main(options.webdir, options.port)
