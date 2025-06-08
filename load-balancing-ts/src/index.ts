import crypto from 'node:crypto';
import util from 'node:util';

class Server {

  constructor(public ip: string, public port: number) {
  }
}

class Resource {
  constructor(public id: string) {
  }
}

const resources = [
  new Resource('eef5938d-3927-460b-bd5c-d5f523e71d35'),
  new Resource('a00fb8c4-36f6-4116-aec0-2bed9d96dd79'),
  new Resource('c3852cbf-feba-4cd6-a4a6-e09eab4f8b95'),
  new Resource('309503ed-2170-4cde-9c1d-76f8855aa9fe'),
  new Resource('e4b78849-97e2-4c63-b5fb-073ccd0f405c'),
  new Resource('f22d4747-fbc9-4dea-a33e-6f026800f730'),
  new Resource('36ca6f25-a048-4002-b340-820148f6c3c0'),
  new Resource('59ddfc71-230f-4174-bc8e-8f29b70c84bc'),
  new Resource('f133e46c-85a7-4abf-8e44-b094739396c2'),
  new Resource('62ef19b3-4fa1-4510-86fe-9c0f5e6ecd5f'),
];

function bigIntHash(content: string) {
  let buffer = crypto.hash('sha1', content, 'buffer');
  return buffer.readBigUInt64LE();
}

const servers = [new Server("127.0.0.1", 1234), new Server("127.0.0.1", 1235)];

const rendezvousHashes = resources
  .map((resource) => {
    const serverAccessOrder = servers.map((server) => {
      return {
        // Concatenate resource hash with server for each resource to get a
        // unique permutation. Note that it is crucial to combine the values and
        // calculate a hash. Computing two hashes and summing them breaks
        // uniformity.
        hash: bigIntHash(`${server.ip}:${server.port}` + resource.id),
        server,
      };
    }).sort((a, b) => (a.hash > b.hash ? -1 : a.hash < b.hash ? 1 : 0));

    return {
      resource,
      serverAccessOrder
    };
  });

// TODO(tatu): Removing, adding servers, when to rebuild?

console.log(util.inspect(rendezvousHashes, { depth: null, colors: true }));
