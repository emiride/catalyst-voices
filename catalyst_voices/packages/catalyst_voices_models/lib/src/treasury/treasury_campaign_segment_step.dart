import 'package:equatable/equatable.dart';

sealed class TreasuryCampaignSegmentStep extends Equatable {
  final int id;
  final bool isEditable;

  TreasuryCampaignSegmentStep({
    required this.id,
    this.isEditable = false,
  });

  @override
  List<Object?> get props => [
        id,
        isEditable,
      ];
}

final class TreasuryCampaignTitle extends TreasuryCampaignSegmentStep {
  TreasuryCampaignTitle({
    required super.id,
    super.isEditable,
  });
}

// Note. Temporary class representing dummy topic
final class TreasuryCampaignTopicX extends TreasuryCampaignSegmentStep {
  final int nr;

  TreasuryCampaignTopicX({
    required super.id,
    required this.nr,
  });

  @override
  List<Object?> get props => super.props + [nr];
}