import 'dart:async';

import 'package:catalyst_voices_blocs/src/account/account_state.dart';
import 'package:catalyst_voices_blocs/src/common/bloc_error_emitter_mixin.dart';
import 'package:catalyst_voices_models/catalyst_voices_models.dart';
import 'package:catalyst_voices_services/catalyst_voices_services.dart';
import 'package:catalyst_voices_shared/catalyst_voices_shared.dart';
import 'package:catalyst_voices_view_models/catalyst_voices_view_models.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

final class AccountCubit extends Cubit<AccountState>
    with BlocErrorEmitterMixin {
  final _logger = Logger('AccountCubit');
  final UserService _userService;

  StreamSubscription<Account?>? _accountSub;

  AccountCubit(
    this._userService,
  ) : super(_buildState(from: _userService.user.activeAccount)) {
    _accountSub = _userService.watchUser
        .map((user) => user.activeAccount)
        .distinct()
        .listen(_handleActiveAccountChange);
  }

  @override
  Future<void> close() async {
    await _accountSub?.cancel();
    _accountSub = null;

    return super.close();
  }

  Future<void> deleteActiveKeychain() async {
    final account = _userService.user.activeAccount;
    if (account != null) {
      await _userService.removeAccount(account);
    }
  }

  Future<void> loadAccountDetails() async {
    // TODO(damian-molinski): Integration
  }

  /// Returns true if updated, false otherwise.
  Future<bool> updateEmail(Email email) async {
    try {
      if (email.isNotValid) {
        return false;
      }

      final activeAccount = _userService.user.activeAccount;
      if (activeAccount != null) {
        await _userService.updateAccount(
          id: activeAccount.catalystId,
          email: email.value,
        );
      }

      emit(state.copyWith(email: email));
      return true;
    } catch (error, stackTrace) {
      _logger.severe('Update email', error, stackTrace);
      emitError(LocalizedException.create(error));
      return false;
    }
  }

  /// Returns true if updated, false otherwise.
  Future<bool> updateUsername(Username username) async {
    try {
      if (username.isNotValid) {
        return false;
      }

      final activeAccount = _userService.user.activeAccount;
      if (activeAccount != null) {
        final value = username.value;

        await _userService.updateAccount(
          id: activeAccount.catalystId,
          username: value.isNotEmpty ? Optional(value) : const Optional.empty(),
        );
      }

      emit(state.copyWith(username: username));
      return true;
    } catch (error, stackTrace) {
      _logger.severe('Update username', error, stackTrace);
      emitError(LocalizedException.create(error));
      return false;
    }
  }

  void _handleActiveAccountChange(Account? account) {
    emit(_buildState(from: account));
  }

  static AccountState _buildState({Account? from}) {
    final roles = from?.roles ?? const {};
    final catalystId = from?.catalystId;

    final accountRolesItems = AccountRole.values
        .where((role) => !role.isHidden)
        .map(
          (role) => MyAccountRoleItem(
            type: role,
            isSelected: roles.contains(role),
          ),
        )
        .toList();

    return AccountState(
      // Note. account status is not supported for f14.
      status: const None(),
      catalystId: catalystId,
      username: Username.pure(catalystId?.username ?? ''),
      email: Email.pure(from?.email ?? ''),
      roles: AccountRolesState(
        items: accountRolesItems,
        canAddRole: accountRolesItems.any((item) => !item.isSelected),
      ),
      walletConnected: from?.stakeAddress ?? '',
    );
  }
}
