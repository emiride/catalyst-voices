import 'package:equatable/equatable.dart';
import 'package:flutter_quill/flutter_quill.dart';

class WorkspaceProposalSegmentStep extends Equatable {
  final int id;
  final String title;
  final String? description;
  final RichTextParams? richTextParams;
  final bool isEditable;

  const WorkspaceProposalSegmentStep({
    required this.id,
    required this.title,
    this.description,
    this.richTextParams,
    this.isEditable = false,
  }) : assert(
          description != null || richTextParams != null,
          'Make sure description or document are provided',
        );

  @override
  List<Object?> get props => [
        id,
        title,
        description,
        richTextParams,
        isEditable,
      ];
}

class RichTextParams {
  final Document document;
  final int? charsLimit;

  RichTextParams({
    required this.document,
    this.charsLimit,
  });
}
