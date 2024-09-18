import 'package:catalyst_voices/pages/workspace/proposal_details.dart';
import 'package:catalyst_voices/pages/workspace/proposal_navigation_panel.dart';
import 'package:catalyst_voices/pages/workspace/proposal_segment_controller.dart';
import 'package:catalyst_voices/pages/workspace/proposal_setup_panel.dart';
import 'package:catalyst_voices/pages/workspace/rich_text/answer.dart';
import 'package:catalyst_voices/pages/workspace/rich_text/bonus_mark_up.dart';
import 'package:catalyst_voices/pages/workspace/rich_text/delivery_and_accountability.dart';
import 'package:catalyst_voices/pages/workspace/rich_text/feasibility_checks.dart';
import 'package:catalyst_voices/pages/workspace/rich_text/problem_statement.dart';
import 'package:catalyst_voices/pages/workspace/rich_text/public_description.dart';
import 'package:catalyst_voices/pages/workspace/rich_text/solution_statement.dart';
import 'package:catalyst_voices/pages/workspace/rich_text/title.dart';
import 'package:catalyst_voices/pages/workspace/rich_text/value_for_money.dart';
import 'package:catalyst_voices/widgets/widgets.dart';
import 'package:catalyst_voices_models/catalyst_voices_models.dart';
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';

const _setupSegmentId = 'setup';
const _summarySegmentId = 'summary';
const _solutionSegmentId = 'solution';
const _impactSegmentId = 'impact';
const _capabilityAndFeasibilitySegmentId = 'capabilityAndFeasibility';

var _proposalNavigation = WorkspaceProposalNavigation(
  segments: [
    WorkspaceProposalSetup(
      id: _setupSegmentId,
      steps: [
        WorkspaceProposalSegmentStep(
          id: 0,
          title: 'Title',
          document: Document.fromJson(title),
        ),
      ],
    ),
    WorkspaceProposalSummary(
      id: _summarySegmentId,
      steps: [
        WorkspaceProposalSegmentStep(
          id: 0,
          title: 'Problem statement',
          document: Document.fromJson(problemStatement),
        ),
        WorkspaceProposalSegmentStep(
          id: 1,
          title: 'Solution statement',
          document: Document.fromJson(solutionStatement),
        ),
        WorkspaceProposalSegmentStep(
          id: 2,
          title: 'Public description',
          document: Document.fromJson(publicDescription),
        ),
      ],
    ),
    WorkspaceProposalSolution(
      id: _solutionSegmentId,
      steps: [
        WorkspaceProposalSegmentStep(
          id: 0,
          title: 'Problem perspective',
          document: Document.fromJson(answer),
        ),
        WorkspaceProposalSegmentStep(
          id: 1,
          title: 'Perspective rationale',
          document: Document.fromJson(answer),
        ),
        WorkspaceProposalSegmentStep(
          id: 2,
          title: 'Project engagement',
          document: Document.fromJson(answer),
        ),
      ],
    ),
    WorkspaceProposalImpact(
      id: _impactSegmentId,
      steps: [
        WorkspaceProposalSegmentStep(
          id: 0,
          title: 'Bonus mark-up',
          document: Document.fromJson(bonusMarkUp),
        ),
        WorkspaceProposalSegmentStep(
          id: 1,
          title: 'Value for Money',
          document: Document.fromJson(valueForMoney),
        ),
      ],
    ),
    WorkspaceProposalCapabilityAndFeasibility(
      id: _capabilityAndFeasibilitySegmentId,
      steps: [
        WorkspaceProposalSegmentStep(
          id: 0,
          title: 'Delivery & Accountability',
          document: Document.fromJson(deliveryAndAccountability),
        ),
        WorkspaceProposalSegmentStep(
          id: 1,
          title: 'Feasibility checks',
          document: Document.fromJson(feasibilityChecks),
        ),
      ],
    ),
  ],
);

class WorkspacePage extends StatefulWidget {
  const WorkspacePage({
    super.key,
  });

  @override
  State<WorkspacePage> createState() => _WorkspacePageState();
}

class _WorkspacePageState extends State<WorkspacePage> {
  @override
  Widget build(BuildContext context) {
    return ProposalControllerScope(
      builder: _buildSegmentController,
      child: SpaceScaffold(
        left: ProposalNavigationPanel(
          navigation: _proposalNavigation,
        ),
        right: const ProposalSetupPanel(),
        child: ProposalDetails(
          navigation: _proposalNavigation,
        ),
      ),
    );
  }

  // Only creates initial controller one time
  ProposalController _buildSegmentController(Object segmentId) {
    final value = segmentId == _setupSegmentId
        ? const ProposalControllerStateData(
            selectedItemId: 0,
            isExpanded: true,
          )
        : const ProposalControllerStateData();

    return ProposalController(value);
  }
}
