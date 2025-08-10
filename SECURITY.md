# Security Policy & Bug Bounty Program

Security is our top priority. We value the work of security researchers and believe open, respectful collaboration is the best way to keep algorithms safe over time. If you find an issue, we want reporting, reproduction, and fixing to be straightforward — with fair recognition and prompt payment.

---

### **At a glance**

* **Duration:** 90 days (Phase 1)
* **Total Budget:** €3,000
* **Targets:** P-Hash-512, P-Key, and their WebAssembly demos.
* **Direct Contact:** `valerio@syntaxos.org`

---

### **How to Report a Vulnerability**

Please email all reports to **`valerio@syntaxos.org`**.

We aim for an initial response within **48 hours** and will provide status updates every 7 days until the issue is closed. Please provide a minimal Proof-of-Concept (PoC) with your report.

### **Program Rules & Scope**

#### **In-Scope Vulnerabilities**
* Key recovery or forging signatures/ciphertexts.
* Collisions or preimage attacks below the stated security bounds for P-Hash-512.
* Keystream recovery or nonce misuse that leads to plaintext recovery.
* Practical side-channel attacks (timing, cache) with a reproducible PoC.
* Memory safety bugs in `--release` builds (e.g., Out-of-Bounds, Use-After-Free) or any Undefined Behavior that impacts security.
* Integrity failures (malleability) that violate the documented security model.

#### **Out-of-Scope**
* Volumetric DDoS, rate-limiting, or spam issues.
* Bugs in third-party libraries without a concrete exploit path in our code.
* Theoretical claims without a reproducible PoC.
* Cosmetic issues, typos, or compiler preference complaints.
* Vulnerabilities in modified or unofficial forks of the code.

### **Reward Structure (Phase 1)**

The final severity and payout amount are at the project’s discretion, depending on impact, exploitability, and report quality. The program may end early if the total budget is exhausted.

| Severity | Max Reward | Indicative Findings |
| :--- | :--- | :--- |
| **Critical** | **€3,000** | Private-key recovery; P-Hash-512 collision < $2^{256}$; practical keystream recovery. |
| **High** | **€1,000** | Reproducible side-channel with concrete leakage; decryption or signature forgery. |
| **Medium** | **€500** | Substantial non-constant-time path; panic in `--release`; local DoS with clear security impact. |

### **Legal & Disclosure**

* **Coordinated Disclosure:** Please do not disclose the vulnerability publicly until a fix is released or 90 days after our acceptance, whichever comes first. We maintain a “Hall of Fame” and will credit you for your work (or respect your request for anonymity).
* **Safe Harbor:** We authorize good-faith research that is limited to the scope defined in this policy. We will not pursue legal action for activities performed in good faith under these terms.

---
