# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: helloworld.proto

require 'google/protobuf'

Google::Protobuf::DescriptorPool.generated_pool.build do
  add_file("helloworld.proto", :syntax => :proto3) do
    add_message "example.helloworld.Request" do
      optional :name, :string, 1
      optional :age, :int32, 2
    end
    add_message "example.helloworld.Response" do
      optional :message, :string, 1
    end
  end
end

module Example
  module Helloworld
    Request = Google::Protobuf::DescriptorPool.generated_pool.lookup("example.helloworld.Request").msgclass
    Response = Google::Protobuf::DescriptorPool.generated_pool.lookup("example.helloworld.Response").msgclass
  end
end
