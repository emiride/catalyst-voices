import 'package:catalyst_voices/pages/spaces/drawer/space_header.dart';
import 'package:catalyst_voices/widgets/widgets.dart';
import 'package:catalyst_voices_models/catalyst_voices_models.dart';
import 'package:flutter/material.dart';

class IndividualPrivateCampaigns extends StatelessWidget {
  const IndividualPrivateCampaigns({super.key});

  @override
  Widget build(BuildContext context) {
    return const Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        SpaceHeader(Space.treasury),
        SectionHeader(
          leading: SizedBox(width: 12),
          title: Text('Individual private campaigns'),
        ),
        VoicesNavTile(
          name: 'Fund name 1',
          status: ProposalStatus.ready,
          trailing: MoreOptionsButton(),
        ),
        VoicesNavTile(
          name: 'Campaign 1',
          status: ProposalStatus.draft,
          trailing: MoreOptionsButton(),
        ),
        VoicesNavTile(
          name: 'What happens with a campaign title that is longer that',
          status: ProposalStatus.draft,
          trailing: MoreOptionsButton(),
        ),
      ],
    );
  }
}