import 'package:catalyst_voices/routes/routing/account_route.dart' as account;
import 'package:catalyst_voices/routes/routing/campaign_stage_route.dart'
    as campaign_stage;
import 'package:catalyst_voices/routes/routing/coming_soon_route.dart'
    as coming_soon;
import 'package:catalyst_voices/routes/routing/overall_spaces_route.dart'
    as overall_spaces;
import 'package:catalyst_voices/routes/routing/proposal_builder_route.dart'
    as proposal_builder;
import 'package:catalyst_voices/routes/routing/proposal_route.dart' as proposal;
import 'package:catalyst_voices/routes/routing/spaces_route.dart' as spaces;
import 'package:go_router/go_router.dart';

/// Semantic anchor for generated routes so only this class
/// knows how to work with them.
abstract final class Routes {
  static const currentMilestone = 'm4';

  static final List<RouteBase> routes = [
    ...account.$appRoutes,
    ...coming_soon.$appRoutes,
    ...spaces.$appRoutes,
    ...overall_spaces.$appRoutes,
    ...proposal.$appRoutes,
    ...proposal_builder.$appRoutes,
    ...campaign_stage.$appRoutes,
  ];

  static String get initialLocation {
    return const coming_soon.ComingSoonRoute().location;
  }
}
