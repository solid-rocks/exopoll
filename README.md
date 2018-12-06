About
-----

Exopoll is an experimental ~~but usable~~ implementation of ananymous
end-to-end verifiable electronic voting protocol. It uses exonum blockchain as
a public registry. This allows voters to check that their vote is included
correctly, and everyone to verify final result.

Our protocol is based on the DRE-ip paper. Notable difference is that we use
zkSNARKs to anonymize voters, ensure one-man-one-vote property and protocol integrity.

General Description
-------------------

Our Goals
---------
  - E2E anonymous voting scalable to millions of voters.

Limitations
-----------
  - Nodes may reveal running sum of votes (but are unable to compromise
    voter's anonymity or change their votes).
  - PBFT is not scalabel enough.
