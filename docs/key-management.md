# Cryptographic Key Management

In the context of information security and compliance frameworks like NIST Special Publication 800-53, **cryptographic key management** refers to the comprehensive set of practices, processes, and systems that oversee the full lifecycle of cryptographic keys. This lifecycle spans all stages from key generation, distribution, storage, and rotation, to archival and eventual destruction (cryptoperiod management). 

Effective key management ensures that cryptographic mechanisms, which protect the confidentiality, integrity, and authenticity of sensitive data, remain robust and trustworthy throughout their operational lives. It is the foundational security control that underpins all other uses of encryption within an organization.

The importance of robust key management cannot be overstated, as the security of encrypted data is entirely dependent on the security of the keys used to protect it. A weak or compromised key management framework can render even the strongest encryption algorithms useless. Poorly managed keys expose an organization to significant risks, including data breaches, loss of data integrity, and compliance failures with regulatory standards (such as FISMA, HIPAA, or GDPR). 

By implementing strong controls outlined in guidance like NIST 800-53 controls (e.g., CM-5, SC-12, SC-13), organizations establish a trusted system for safeguarding these critical assets, thereby maintaining secure operations and building verifiable trust in their information systems.

Many people get tripped up in audits: not the crypto itself, but how well the key lifecycle is documented and justified. In this document, it will be addressed in three layers:

* What standards actually require (authoritative controls)
* What auditors typically expect to see
* A practical, accreditation-safe documentation checklist you can actually use


## Authoritative security controls that govern keys & lifetimes

Across NIST, STIG, RTB, and MLS environments,key documentation requirements consistently come from the same control families.  Almost everything maps to NIST 800-53 Rev. 5.

### Key management (primary)

* KM-1 — Cryptographic Key Management Policy
  - You must document:
	- What kinds of keys exist
	- What each key is used for
	- Who is responsible for lifecycle decisions

* KM-2 — Key Generation
  - You must document:
	- How keys are generated
	- Algorithms used
	- Key strength
	- Whether generation is automated or manual

* KM-3 — Key Protection
  - You must document:
	- Where keys are stored
	- How access is restricted
	- Why passphrases are or are not used
	- Protection mechanisms (permissions, SELinux, HSM, TPM)

* KM-4 — Key Lifetime and Rotation
  - You must document:
	- Key validity period
	- Rotation policy (scheduled, event-driven, or “until decommission”)
	- Revocation / replacement procedure
---

### Audit and accountability

* AU-2 — Audit Events
  - You must generate audit records for:
	- Key creation
	- Key rotation
	- Key revocation
	- Key deletion

* AU-3 — Audit Record Contents
  - Audit records must include:
	- What key (identifier)
	- Action taken (created, rotated, retired)
	- When it happened
	- Who or what initiated it

* AU-6 — Audit Review
  - You must be able to demonstrate:
	- How key-related events are reviewed
	- That failures or anomalies are detectable

---

### Configuration management

CM-2 — Baseline Configuration
You must document:
	•	Expected presence of keys
	•	Expected file locations
	•	Expected permissions and labels
	•	What constitutes “configuration drift”

CM-6 — Configuration Settings
You must justify:
	•	Why key settings are what they are
	•	Why defaults were changed (or not)

---

### System integrity & cryptography

SI-7 — Integrity Checks
You must show:
	•	What integrity guarantees the keys provide
	•	What happens if validation fails

SC-12 / SC-13 — Cryptographic Key Establishment & Protection
You must:
	•	Demonstrate FIPS-approved algorithms
	•	Explain system trust boundaries

---

## What auditors and reviewers actually expect to see

This is critical: auditors do not expect perfection, but they do expect explicit intent.

They want answers to these exact questions:

### Key inventory

For each key:
	•	Key name / identifier
	•	Key purpose
	•	Algorithm and size
	•	Storage location
	•	Which system component uses it

Example (simple):
> “Log archive signing key used to sign rotated application logs prior to off-system transfer.”


### Lifetime definition

Auditors want clarity, not dogma.

Acceptable lifetime explanations include:
	•	“Key is rotated every 365 days”
	•	“Key is rotated at system rebuild”
	•	“Key is rotated on compromise, role change, or cryptographic deprecation”

What is not acceptable:
	•	“Key exists until it doesn’t”
	•	“We haven’t thought about it”


### Rotation and revocation procedure

You must explain:
	•	How a new key is generated
	•	How old keys are retired
	•	What happens to previously signed artifacts
	•	How verification continues with historical keys

This can be procedural — it does not have to be automated.

### Protection rationale (especially no passphrases)

This is where your earlier question lands.

You must explicitly explain:
	•	Why automated keys have no passphrase
	•	What compensating controls are in place
	•	Why that is sufficient for this threat model

This explanation is normal, expected, and allowed.

### Key Event visibility

They will expect:
	•	Key Creation events logged
	•	Key Rotation events logged
	•	Failure to sign logged
	•	Verification failures logged

They do NOT expect:
	•	Secrets in logs
	•	Private key material logged

---

## Practical documentation checklist (use this)

Key documentation template

For each signing key:
1.	Key Identifier
	•	Logical name
	•	Filesystem path

2.	Purpose
	•	What is signed
	•	Why signing is required

3.	Algorithm & Parameters
	•	Algorithm (e.g., RSA-3072, ECDSA-P384, Ed25519 — if FIPS-approved where required)
	•	Hash function

4.	Key Generation
	•	Method (manual provisioning / automated first-boot / build pipeline)
	•	Entropy source
	•	FIPS mode considerations

5.	Key Storage & Protection
	•	Location on disk
	•	Ownership & permissions
	•	SELinux type/domain restrictions
	•	Explanation for passphrase or no passphrase

6.	Key Lifetime
	•	Defined validity period or policy
	•	Rotation triggers

7.	Key Rotation & Revocation
	•	How new keys are introduced
	•	How old keys are retired
	•	How historic signatures are verified

8.	Auditing & Logging
	•	What key events are logged
	•	Log location
	•	Review process

9.	Failure Handling
	•	What happens if signing fails
	•	What happens if verification fails

10.	Residual Risk Statement
	•	What risks remain
	•	Why they are acceptable

Example lifetime policy language (you can reuse):
> “System-generated signing keys are valid for a maximum of 12 months and are rotated during scheduled maintenance or system rebuilds. Keys may also be replaced immediately following suspected compromise, cryptographic deprecation, or role change. Historical public keys are retained to support verification of previously signed artifacts.”

## Why this satisfies MLS / RTB thinking

This approach aligns with high-assurance expectations because:
* Keys are purpose-bound
* Lifetimes are explicit
* Protection mechanisms are documented and layered
* Automation is intentional, not implicit
* Residual risk is acknowledged, not ignored

RTB does not require:
* Online CAs
* Mandatory HSMs
* Perfect secrecy

It requires clarity, control, and traceability.

Security controls require you to:
* Inventory keys
* Document purpose
* Define lifetime
* Justify protection mechanisms
* Log lifecycle events
* Explain failures and residual risk

They do not require:
* Passphrases for unattended services
* Infinite rotation
* Magical automation

