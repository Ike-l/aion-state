use crate::prelude::{Host, Owner};

pub mod host;
pub mod owner;

pub mod reception_input;
pub mod reception_result;

/// Applies `Owner` semantics, then `Host` semantics
pub struct Reception<RS, AS, OS, WS, BS, CS> {
    owner: Owner<OS, WS, BS, CS>,
    host: Host<RS, AS>
}

// impl<
//     RS: ReservationStorage<AccessStorage = AS>, 
//     AS: AccessStorage + Default,
//     OS: CredentialStorage,
//     WS: WhitelistStorage,
//     BS: BlacklistStorage,
//     CS: ControlStorage,
// > Reception<RS, AS, OS, WS, BS, CS> 
//     where 
//         RS::ReserverId: Debug + PartialEq,
//         AS::Access: Debug + Accessor,
// {
//     /// If owner permits then can check if host also permits
//     pub fn permits_access(
//         &self,
//         ReceptionAccessPermissionInput {
//             reserver_id, value_id, access, value_password
//         }: ReceptionAccessPermissionInput<'_, RS::ReserverId, AS::ValueId, AS::Access, PS::ValuePassword>
//     ) -> ReceptionAccessPermissionResult {
//         trace_function!("Reception Permits Access");

//         let owner_access_permission = self.owner.permits_access(OwnerAccessPermissionInput { value_id, value_password, access });
//         if owner_access_permission.ok() {
//             ReceptionAccessPermissionResult::Host(self.host.permits_access(HostAccessPermissionInput { reserver_id, access_id: value_id, access }))
//         } else {
//             ReceptionAccessPermissionResult::Denied
//         }
//     }

//     /// If owner permits then reserve with host
//     pub fn reserve(
//         &mut self,
//         ReceptionReservationInput {
//             value_id, value_password, access, reserver_id
//         }: ReceptionReservationInput<'_, AS::ValueId, PS::ValuePassword, AS::Access, RS::ReserverId>
//     ) -> ReceptionReservationResult {
//         let owner_access_permission = self.owner.permits_access(OwnerAccessPermissionInput { value_id: &value_id, value_password, access: &access });
//         if owner_access_permission.ok() {
//             // record reservation with reserver "owning" the access_id if host is okay?
//             // does this mean there are 2 sides?
//             // so need Authenticator -> Host -> Owner
//             ReceptionReservationResult::Host(self.host.reserve(ReserveInput { reserver_id, access_id: value_id, access }))
//         } else {
//             ReceptionReservationResult::Denied
//         }
//     }

//     pub fn unreserve(
//         &mut self,
//         ReceptionUnreserveInput {
//             reserver_id, access_id, access
//         }: ReceptionUnreserveInput<'_, RS::ReserverId, AS::ValueId, AS::Access>
//     ) -> ReceptionUnreserveResult {
//         // if self.owner.authenticate().ok() {
//             ReceptionUnreserveResult::Host(self.host.unreserve(UnreserveInput { reserver_id, access_id, access }))
//         // } else {
//         //     ReceptionUnreserveResult::Denied
//         // }
//     }

//     // pub fn locks
//     // checks for any reservation or accesses 
//     // if !self.host.has_stakeholder {
//     //  self.owner.lock()
//     // }
//     // if self.host.allows_ownership
//     // if !(self.host.has_reservations or self.host.has_access)


//     // /// `pass through` to owner- since owner generates passwords
//     // /// 
//     // /// Understanding this is as simple as understanding the order of semantics
//     // /// 
//     // /// Host semantics are after owner the host should not "add" any semantics to the password
//     pub fn generate_password(
//         &mut self,
//         ReceptionPasswordGeneratorInput {
//             owner_id, owner_password, value_id, access, policy
//         }: ReceptionPasswordGeneratorInput<'_, OS::OwnerId, OS::OwnerPassword, LS::ValueId, PS::Access, PS::GenerationPolicy>
//     ) -> ReceptionPasswordGeneratorResult<PS::ValuePassword> {
//         trace_function!("Reception Generate Password");

//         // To generate passwords does the `host` have to do anything? 
//         // When generating a password iow do i need `reservation` or `accesses` semantics?
//         let result = self.owner.generate_password(OwnerPasswordGeneratorInput { owner_id, owner_password, value_id, access, policy });
//         ReceptionPasswordGeneratorResult::Owner(result)
//     }
// }