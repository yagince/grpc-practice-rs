$LOAD_PATH.unshift(File.expand_path("../", __FILE__))

require 'helloworld_services_pb'

stub = Example::Helloworld::Helloworld::Stub.new('localhost:50051', :this_channel_is_insecure)

req = Example::Helloworld::Request.new(name: "hoge", age: 20000)

pp stub.call(req)
