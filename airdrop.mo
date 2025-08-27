import SNSLedger "canister:your_sns_ledger_canister_id";

actor Airdrop {
  stable var participants : [(Principal, Nat)] = []; // (address, ICP contributed)
  stable var treasury : Nat = 0; // $BEAR tokens available

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
    let totalICP = participants.foldLeft(0, func(acc, x) = acc + x.1);
    let airdropAmount = treasury / 10; // 10% monthly
    for (p in participants.vals()) {
      let percent = Float.fromInt(p.1) / Float.fromInt(totalICP);
      let payout = Nat.fromFloat(Float.fromInt(airdropAmount) * percent);
      ignore SNSLedger.transfer(p.0, payout); // transfer $BEAR tokens
    };
    treasury -= airdropAmount;
  };

  // Timer setup (call this once after deployment)
  public func startAirdropTimer() : async () {
    Timer.setTimer(
      #interval(30 * 24 * 60 * 60), // 30 days
      func () { ignore monthlyAirdrop() }
    );
  };
}
