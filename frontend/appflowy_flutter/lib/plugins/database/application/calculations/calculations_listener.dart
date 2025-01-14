import 'dart:async';
import 'dart:typed_data';

import 'package:appflowy/core/notification/grid_notification.dart';
import 'package:appflowy_backend/protobuf/flowy-database2/protobuf.dart';
import 'package:appflowy_backend/protobuf/flowy-error/errors.pb.dart';
import 'package:dartz/dartz.dart';
import 'package:flowy_infra/notifier.dart';

typedef UpdateCalculationValue
    = Either<CalculationChangesetNotificationPB, FlowyError>;

class CalculationsListener {
  CalculationsListener({required this.viewId});

  final String viewId;

  PublishNotifier<UpdateCalculationValue>? _calculationNotifier =
      PublishNotifier();
  DatabaseNotificationListener? _listener;

  void start({
    required void Function(UpdateCalculationValue) onCalculationChanged,
  }) {
    _calculationNotifier?.addPublishListener(onCalculationChanged);
    _listener = DatabaseNotificationListener(
      objectId: viewId,
      handler: _handler,
    );
  }

  void _handler(
    DatabaseNotification ty,
    Either<Uint8List, FlowyError> result,
  ) {
    switch (ty) {
      case DatabaseNotification.DidUpdateCalculation:
        _calculationNotifier?.value = result.fold(
          (payload) => left(
            CalculationChangesetNotificationPB.fromBuffer(payload),
          ),
          (err) => right(err),
        );
      default:
        break;
    }
  }

  Future<void> stop() async {
    await _listener?.stop();
    _calculationNotifier?.dispose();
    _calculationNotifier = null;
  }
}
