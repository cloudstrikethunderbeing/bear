import SNSLedger "canister:your_sns_ledger_canister_id";
import Timer "mo:base/Timer";

actor Airdrop {
  stable var participants : [(Principal, Nat)] = [];
  stable var treasury : Nat = 0;

  public func addContribution(p : Principal, icp : Nat) : async () {
    let idx = participants.findIndex(func x = x.0 == p);
    if (idx == null) {
      participants := participants # [(p, icp)];
    } else {
      participants[idx!] := (p, participants[idx!].1 + icp);
    }
  };

  public func setTreasury(amount : Nat) : async () {
    treasury := amount;
  };

  public func monthlyAirdrop() : async () {
    let vals = participants.vals();
    var i = 0;
    let n = vals.size();
    while (i < n) {
      let p = vals[i];
      let payout = p.1;
      if (treasury >= payout) {
        ignore SNSLedger.transfer(p.0, payout);
        treasury -= payout;
      } else {
        ignore SNSLedger.transfer(p.0, treasury);
        treasury := 0;
        break;
      }
      i += 1;
    }
  }

  public func startAirdropTimer() : async () {
    Timer.setTimer(
      #interval(30 * 24 * 60 * 60),
      func () { ignore monthlyAirdrop() }
    );
  }
}