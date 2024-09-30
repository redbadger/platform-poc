import * as civo from "@pulumi/civo";
import * as pulumi from "@pulumi/pulumi";
import * as std from "@pulumi/std";

const project = "platform-poc-vm";

const network_label = `${project}-network`;

const network = new civo.Network(project, {
  label: network_label,
});

const firewall = new civo.Firewall(project, {
  name: `${project}-firewall`,
  createDefaultRules: true,
  networkId: network.id,
});

const debian = civo.getDiskImage({
  filters: [
    {
      key: "name",
      values: ["debian-11"],
    },
  ],
});

const instance1 = new civo.Instance(project, {
  hostname: `${project}-instance-1`,
  tags: [project],
  notes: "Red Badger Platform PoC - instance 1",
  firewallId: firewall.id,
  networkId: network.id,
  size: "g3.xsmall",
  diskImage: debian.then((debian) => debian.diskimages?.[0]?.id),
  initialUser: "civo",
  script: std.file({ input: "./init.sh" }).then((invoke) => invoke.result),
});

export const instanceId = instance1.id;
export const instanceIp = instance1.publicIp;
export const instancePassword = instance1.initialPassword;

// const db = new civo.Database(project, {
//   name: `${project}-db`,
//   engine: "PostgreSQL",
//   version: "14",
//   nodes: 1,
//   size: "g3.db.xsmall",
//   firewallId: firewall.id,
//   networkId: network.id,
// });

// export const dbId = db.id;
// export const dbPassword = db.password;
// export const dbUsername = db.username;
// export const dbEndpoint = db.endpoint;
