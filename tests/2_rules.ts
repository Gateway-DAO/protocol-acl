// import { expect } from "chai";
// import { file_pda, WRITE_PERM, rule_pda, READ_PERM } from "./common";
// import { FILE_ID, PROGRAM, PROVIDER, namespaces, RoleType } from "./constants";

// describe("2.- Rules", () => {
//   let filePDA = null; // Populated on before() block
//   const role1 = [RoleType.Update];
//   const resource1 = "Admin";
//   const permission1 = "*";
//   let rule1PDA = null; // Populated on before() block

//   before(async () => {
//     filePDA = await file_pda();
//     rule1PDA = await rule_pda(role1, resource1, permission1);
//   });

//   it("Add rule", async () => {
//     let listener = null;
//     let [event, _]: any = await new Promise((resolve, reject) => {
//       listener = PROGRAM.addEventListener("RulesChanged", (event, slot) => {
//         PROGRAM.removeEventListener(listener);
//         resolve([event, slot]);
//       });
//       PROGRAM.methods
//         .addRule({
//           namespace: namespaces.Rule,
//           resource: resource1,
//           permission: permission1,
//           roles: role1,
//           expiresAt: null,
//         })
//         .accounts({
//           rule: rule1PDA,
//           solGatewayFile: filePDA,
//           solGatewayRole: null,
//           solGatewayRule: null,
//           solGatewayRule2: null,
//           solGatewayToken: null,
//           solGatewayMetadata: null,
//           solGatewaySeed: null,
//         })
//         .rpc();
//       // Break infinite loop in case it fails:
//       setTimeout(() => {
//         reject(new Error("Failed to add rule"));
//       }, 2000);
//     });
//     let rule = await PROGRAM.account.rule.fetch(rule1PDA);
//     expect(rule.fileId.toBase58()).to.equal(FILE_ID.toBase58());
//     expect(rule.fileId.toBase58()).to.equal(event.fileId.toBase58());
//     expect(rule.roles).to.equal(role1);
//     expect(rule.resource).to.equal(resource1);
//     expect(rule.permission).to.equal(permission1);

//     // Add Write rule
//     await PROGRAM.methods
//       .addRule({
//         namespace: namespaces.Rule,
//         roles: [RoleType.Update],
//         resource: WRITE_PERM.resource,
//         permission: WRITE_PERM.permission,
//         expiresAt: null,
//       })
//       .accounts({
//         rule: await rule_pda(
//           WRITE_PERM.role,
//           WRITE_PERM.resource,
//           WRITE_PERM.permission
//         ),
//         solGatewayFile: filePDA,
//         solGatewayRole: null,
//         solGatewayRule: null,
//         solGatewayRule2: null,
//         solGatewayToken: null,
//         solGatewayMetadata: null,
//         solGatewaySeed: null,
//       })
//       .rpc();

//     // Add Read rule
//     await PROGRAM.methods
//       .addRule({
//         namespace: namespaces.Rule,
//         role: READ_PERM.role,
//         resource: READ_PERM.resource,
//         permission: READ_PERM.permission,
//         expiresAt: null,
//       })
//       .accounts({
//         rule: await rule_pda(
//           READ_PERM.role,
//           READ_PERM.resource,
//           READ_PERM.permission
//         ),
//         solGatewayFile: filePDA,
//         solGatewayRole: null,
//         solGatewayRule: null,
//         solGatewayRule2: null,
//         solGatewayToken: null,
//         solGatewayMetadata: null,
//         solGatewaySeed: null,
//       })
//       .rpc();
//   });

//   it("Delete rule", async () => {
//     await PROGRAM.methods
//       .deleteRule()
//       .accounts({
//         rule: rule1PDA,
//         collector: PROVIDER.wallet.publicKey,
//         solGatewayFile: filePDA,
//         solGatewayRole: null,
//         solGatewayRule: null,
//         solGatewayRule2: null,
//         solGatewayToken: null,
//         solGatewayMetadata: null,
//         solGatewaySeed: null,
//       })
//       .rpc();
//     try {
//       await PROGRAM.account.rule.fetch(rule1PDA);
//       throw new Error("The rule should have been deleted at this point!");
//     } catch (_err) {
//       expect(_err.toString()).to.include(
//         "Account does not exist or has no data"
//       );
//     }
//   });
// });
