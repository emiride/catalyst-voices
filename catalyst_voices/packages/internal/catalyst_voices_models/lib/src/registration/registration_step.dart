import 'package:catalyst_voices_models/src/registration/create_keychain_stage.dart';
import 'package:catalyst_voices_models/src/registration/recover_seed_phrase_stage.dart';
import 'package:catalyst_voices_models/src/registration/wallet_link_stage.dart';
import 'package:equatable/equatable.dart';

/// Represents only step of registration, not stage of this step.
sealed class RegistrationStep extends Equatable {
  const RegistrationStep();

  bool get isRegistrationFlow => false;

  bool get isRecoverFlow => false;

  @override
  List<Object?> get props => [];
}

final class GetStartedStep extends RegistrationStep {
  const GetStartedStep();
}

final class RecoverMethodStep extends RegistrationStep {
  const RecoverMethodStep();
}

final class SeedPhraseRecoverStep extends RegistrationStep {
  final RecoverSeedPhraseStage stage;

  const SeedPhraseRecoverStep({
    this.stage = RecoverSeedPhraseStage.seedPhraseInstructions,
  });

  @override
  bool get isRecoverFlow => true;

  @override
  List<Object?> get props => [stage];
}

final class CreateKeychainStep extends RegistrationStep {
  final CreateKeychainStage stage;

  const CreateKeychainStep({
    this.stage = CreateKeychainStage.splash,
  });

  @override
  bool get isRegistrationFlow => true;

  @override
  List<Object?> get props => [stage];
}

final class FinishAccountCreationStep extends RegistrationStep {
  const FinishAccountCreationStep();

  @override
  bool get isRegistrationFlow => true;
}

final class WalletLinkStep extends RegistrationStep {
  final WalletLinkStage stage;

  const WalletLinkStep({
    this.stage = WalletLinkStage.intro,
  });

  @override
  bool get isRegistrationFlow => true;

  @override
  List<Object?> get props => [stage];
}

final class AccountCompletedStep extends RegistrationStep {
  const AccountCompletedStep();
}
