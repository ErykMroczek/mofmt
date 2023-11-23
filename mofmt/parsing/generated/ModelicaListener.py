# Generated from grammar/Modelica.g4 by ANTLR 4.13.0
from antlr4 import *

if "." in __name__:
    from .Modelica import Modelica
else:
    from Modelica import Modelica


# This class defines a complete listener for a parse tree produced by Modelica.
class ModelicaListener(ParseTreeListener):
    # Enter a parse tree produced by Modelica#stored_definition.
    def enterStored_definition(self, ctx: Modelica.Stored_definitionContext):
        pass

    # Exit a parse tree produced by Modelica#stored_definition.
    def exitStored_definition(self, ctx: Modelica.Stored_definitionContext):
        pass

    # Enter a parse tree produced by Modelica#class_definition.
    def enterClass_definition(self, ctx: Modelica.Class_definitionContext):
        pass

    # Exit a parse tree produced by Modelica#class_definition.
    def exitClass_definition(self, ctx: Modelica.Class_definitionContext):
        pass

    # Enter a parse tree produced by Modelica#class_prefixes.
    def enterClass_prefixes(self, ctx: Modelica.Class_prefixesContext):
        pass

    # Exit a parse tree produced by Modelica#class_prefixes.
    def exitClass_prefixes(self, ctx: Modelica.Class_prefixesContext):
        pass

    # Enter a parse tree produced by Modelica#class_specifier.
    def enterClass_specifier(self, ctx: Modelica.Class_specifierContext):
        pass

    # Exit a parse tree produced by Modelica#class_specifier.
    def exitClass_specifier(self, ctx: Modelica.Class_specifierContext):
        pass

    # Enter a parse tree produced by Modelica#long_class_specifier.
    def enterLong_class_specifier(self, ctx: Modelica.Long_class_specifierContext):
        pass

    # Exit a parse tree produced by Modelica#long_class_specifier.
    def exitLong_class_specifier(self, ctx: Modelica.Long_class_specifierContext):
        pass

    # Enter a parse tree produced by Modelica#end_clause.
    def enterEnd_clause(self, ctx: Modelica.End_clauseContext):
        pass

    # Exit a parse tree produced by Modelica#end_clause.
    def exitEnd_clause(self, ctx: Modelica.End_clauseContext):
        pass

    # Enter a parse tree produced by Modelica#short_class_specifier.
    def enterShort_class_specifier(self, ctx: Modelica.Short_class_specifierContext):
        pass

    # Exit a parse tree produced by Modelica#short_class_specifier.
    def exitShort_class_specifier(self, ctx: Modelica.Short_class_specifierContext):
        pass

    # Enter a parse tree produced by Modelica#der_class_specifier.
    def enterDer_class_specifier(self, ctx: Modelica.Der_class_specifierContext):
        pass

    # Exit a parse tree produced by Modelica#der_class_specifier.
    def exitDer_class_specifier(self, ctx: Modelica.Der_class_specifierContext):
        pass

    # Enter a parse tree produced by Modelica#base_prefix.
    def enterBase_prefix(self, ctx: Modelica.Base_prefixContext):
        pass

    # Exit a parse tree produced by Modelica#base_prefix.
    def exitBase_prefix(self, ctx: Modelica.Base_prefixContext):
        pass

    # Enter a parse tree produced by Modelica#enumerations.
    def enterEnumerations(self, ctx: Modelica.EnumerationsContext):
        pass

    # Exit a parse tree produced by Modelica#enumerations.
    def exitEnumerations(self, ctx: Modelica.EnumerationsContext):
        pass

    # Enter a parse tree produced by Modelica#enum_list.
    def enterEnum_list(self, ctx: Modelica.Enum_listContext):
        pass

    # Exit a parse tree produced by Modelica#enum_list.
    def exitEnum_list(self, ctx: Modelica.Enum_listContext):
        pass

    # Enter a parse tree produced by Modelica#enumeration_literal.
    def enterEnumeration_literal(self, ctx: Modelica.Enumeration_literalContext):
        pass

    # Exit a parse tree produced by Modelica#enumeration_literal.
    def exitEnumeration_literal(self, ctx: Modelica.Enumeration_literalContext):
        pass

    # Enter a parse tree produced by Modelica#composition.
    def enterComposition(self, ctx: Modelica.CompositionContext):
        pass

    # Exit a parse tree produced by Modelica#composition.
    def exitComposition(self, ctx: Modelica.CompositionContext):
        pass

    # Enter a parse tree produced by Modelica#class_annotation.
    def enterClass_annotation(self, ctx: Modelica.Class_annotationContext):
        pass

    # Exit a parse tree produced by Modelica#class_annotation.
    def exitClass_annotation(self, ctx: Modelica.Class_annotationContext):
        pass

    # Enter a parse tree produced by Modelica#external_element.
    def enterExternal_element(self, ctx: Modelica.External_elementContext):
        pass

    # Exit a parse tree produced by Modelica#external_element.
    def exitExternal_element(self, ctx: Modelica.External_elementContext):
        pass

    # Enter a parse tree produced by Modelica#language_specification.
    def enterLanguage_specification(self, ctx: Modelica.Language_specificationContext):
        pass

    # Exit a parse tree produced by Modelica#language_specification.
    def exitLanguage_specification(self, ctx: Modelica.Language_specificationContext):
        pass

    # Enter a parse tree produced by Modelica#external_function_call.
    def enterExternal_function_call(self, ctx: Modelica.External_function_callContext):
        pass

    # Exit a parse tree produced by Modelica#external_function_call.
    def exitExternal_function_call(self, ctx: Modelica.External_function_callContext):
        pass

    # Enter a parse tree produced by Modelica#external_function_args.
    def enterExternal_function_args(self, ctx: Modelica.External_function_argsContext):
        pass

    # Exit a parse tree produced by Modelica#external_function_args.
    def exitExternal_function_args(self, ctx: Modelica.External_function_argsContext):
        pass

    # Enter a parse tree produced by Modelica#initial_element_list.
    def enterInitial_element_list(self, ctx: Modelica.Initial_element_listContext):
        pass

    # Exit a parse tree produced by Modelica#initial_element_list.
    def exitInitial_element_list(self, ctx: Modelica.Initial_element_listContext):
        pass

    # Enter a parse tree produced by Modelica#public_element_list.
    def enterPublic_element_list(self, ctx: Modelica.Public_element_listContext):
        pass

    # Exit a parse tree produced by Modelica#public_element_list.
    def exitPublic_element_list(self, ctx: Modelica.Public_element_listContext):
        pass

    # Enter a parse tree produced by Modelica#protected_element_list.
    def enterProtected_element_list(self, ctx: Modelica.Protected_element_listContext):
        pass

    # Exit a parse tree produced by Modelica#protected_element_list.
    def exitProtected_element_list(self, ctx: Modelica.Protected_element_listContext):
        pass

    # Enter a parse tree produced by Modelica#element_list.
    def enterElement_list(self, ctx: Modelica.Element_listContext):
        pass

    # Exit a parse tree produced by Modelica#element_list.
    def exitElement_list(self, ctx: Modelica.Element_listContext):
        pass

    # Enter a parse tree produced by Modelica#element.
    def enterElement(self, ctx: Modelica.ElementContext):
        pass

    # Exit a parse tree produced by Modelica#element.
    def exitElement(self, ctx: Modelica.ElementContext):
        pass

    # Enter a parse tree produced by Modelica#import_clause.
    def enterImport_clause(self, ctx: Modelica.Import_clauseContext):
        pass

    # Exit a parse tree produced by Modelica#import_clause.
    def exitImport_clause(self, ctx: Modelica.Import_clauseContext):
        pass

    # Enter a parse tree produced by Modelica#import_list.
    def enterImport_list(self, ctx: Modelica.Import_listContext):
        pass

    # Exit a parse tree produced by Modelica#import_list.
    def exitImport_list(self, ctx: Modelica.Import_listContext):
        pass

    # Enter a parse tree produced by Modelica#declaration_clause.
    def enterDeclaration_clause(self, ctx: Modelica.Declaration_clauseContext):
        pass

    # Exit a parse tree produced by Modelica#declaration_clause.
    def exitDeclaration_clause(self, ctx: Modelica.Declaration_clauseContext):
        pass

    # Enter a parse tree produced by Modelica#extends_clause.
    def enterExtends_clause(self, ctx: Modelica.Extends_clauseContext):
        pass

    # Exit a parse tree produced by Modelica#extends_clause.
    def exitExtends_clause(self, ctx: Modelica.Extends_clauseContext):
        pass

    # Enter a parse tree produced by Modelica#constraining_clause.
    def enterConstraining_clause(self, ctx: Modelica.Constraining_clauseContext):
        pass

    # Exit a parse tree produced by Modelica#constraining_clause.
    def exitConstraining_clause(self, ctx: Modelica.Constraining_clauseContext):
        pass

    # Enter a parse tree produced by Modelica#class_or_inheritance_modification.
    def enterClass_or_inheritance_modification(
        self, ctx: Modelica.Class_or_inheritance_modificationContext
    ):
        pass

    # Exit a parse tree produced by Modelica#class_or_inheritance_modification.
    def exitClass_or_inheritance_modification(
        self, ctx: Modelica.Class_or_inheritance_modificationContext
    ):
        pass

    # Enter a parse tree produced by Modelica#argument_or_inheritance_modification_list.
    def enterArgument_or_inheritance_modification_list(
        self, ctx: Modelica.Argument_or_inheritance_modification_listContext
    ):
        pass

    # Exit a parse tree produced by Modelica#argument_or_inheritance_modification_list.
    def exitArgument_or_inheritance_modification_list(
        self, ctx: Modelica.Argument_or_inheritance_modification_listContext
    ):
        pass

    # Enter a parse tree produced by Modelica#inheritance_modification.
    def enterInheritance_modification(
        self, ctx: Modelica.Inheritance_modificationContext
    ):
        pass

    # Exit a parse tree produced by Modelica#inheritance_modification.
    def exitInheritance_modification(
        self, ctx: Modelica.Inheritance_modificationContext
    ):
        pass

    # Enter a parse tree produced by Modelica#component_clause.
    def enterComponent_clause(self, ctx: Modelica.Component_clauseContext):
        pass

    # Exit a parse tree produced by Modelica#component_clause.
    def exitComponent_clause(self, ctx: Modelica.Component_clauseContext):
        pass

    # Enter a parse tree produced by Modelica#type_prefix.
    def enterType_prefix(self, ctx: Modelica.Type_prefixContext):
        pass

    # Exit a parse tree produced by Modelica#type_prefix.
    def exitType_prefix(self, ctx: Modelica.Type_prefixContext):
        pass

    # Enter a parse tree produced by Modelica#component_list.
    def enterComponent_list(self, ctx: Modelica.Component_listContext):
        pass

    # Exit a parse tree produced by Modelica#component_list.
    def exitComponent_list(self, ctx: Modelica.Component_listContext):
        pass

    # Enter a parse tree produced by Modelica#component_declaration.
    def enterComponent_declaration(self, ctx: Modelica.Component_declarationContext):
        pass

    # Exit a parse tree produced by Modelica#component_declaration.
    def exitComponent_declaration(self, ctx: Modelica.Component_declarationContext):
        pass

    # Enter a parse tree produced by Modelica#declaration.
    def enterDeclaration(self, ctx: Modelica.DeclarationContext):
        pass

    # Exit a parse tree produced by Modelica#declaration.
    def exitDeclaration(self, ctx: Modelica.DeclarationContext):
        pass

    # Enter a parse tree produced by Modelica#modification.
    def enterModification(self, ctx: Modelica.ModificationContext):
        pass

    # Exit a parse tree produced by Modelica#modification.
    def exitModification(self, ctx: Modelica.ModificationContext):
        pass

    # Enter a parse tree produced by Modelica#modification_expression.
    def enterModification_expression(
        self, ctx: Modelica.Modification_expressionContext
    ):
        pass

    # Exit a parse tree produced by Modelica#modification_expression.
    def exitModification_expression(self, ctx: Modelica.Modification_expressionContext):
        pass

    # Enter a parse tree produced by Modelica#class_modification.
    def enterClass_modification(self, ctx: Modelica.Class_modificationContext):
        pass

    # Exit a parse tree produced by Modelica#class_modification.
    def exitClass_modification(self, ctx: Modelica.Class_modificationContext):
        pass

    # Enter a parse tree produced by Modelica#argument_list.
    def enterArgument_list(self, ctx: Modelica.Argument_listContext):
        pass

    # Exit a parse tree produced by Modelica#argument_list.
    def exitArgument_list(self, ctx: Modelica.Argument_listContext):
        pass

    # Enter a parse tree produced by Modelica#argument.
    def enterArgument(self, ctx: Modelica.ArgumentContext):
        pass

    # Exit a parse tree produced by Modelica#argument.
    def exitArgument(self, ctx: Modelica.ArgumentContext):
        pass

    # Enter a parse tree produced by Modelica#element_modification_or_replaceable.
    def enterElement_modification_or_replaceable(
        self, ctx: Modelica.Element_modification_or_replaceableContext
    ):
        pass

    # Exit a parse tree produced by Modelica#element_modification_or_replaceable.
    def exitElement_modification_or_replaceable(
        self, ctx: Modelica.Element_modification_or_replaceableContext
    ):
        pass

    # Enter a parse tree produced by Modelica#element_modification.
    def enterElement_modification(self, ctx: Modelica.Element_modificationContext):
        pass

    # Exit a parse tree produced by Modelica#element_modification.
    def exitElement_modification(self, ctx: Modelica.Element_modificationContext):
        pass

    # Enter a parse tree produced by Modelica#element_redeclaration.
    def enterElement_redeclaration(self, ctx: Modelica.Element_redeclarationContext):
        pass

    # Exit a parse tree produced by Modelica#element_redeclaration.
    def exitElement_redeclaration(self, ctx: Modelica.Element_redeclarationContext):
        pass

    # Enter a parse tree produced by Modelica#element_replaceable.
    def enterElement_replaceable(self, ctx: Modelica.Element_replaceableContext):
        pass

    # Exit a parse tree produced by Modelica#element_replaceable.
    def exitElement_replaceable(self, ctx: Modelica.Element_replaceableContext):
        pass

    # Enter a parse tree produced by Modelica#short_component_clause.
    def enterShort_component_clause(self, ctx: Modelica.Short_component_clauseContext):
        pass

    # Exit a parse tree produced by Modelica#short_component_clause.
    def exitShort_component_clause(self, ctx: Modelica.Short_component_clauseContext):
        pass

    # Enter a parse tree produced by Modelica#short_component_declaration.
    def enterShort_component_declaration(
        self, ctx: Modelica.Short_component_declarationContext
    ):
        pass

    # Exit a parse tree produced by Modelica#short_component_declaration.
    def exitShort_component_declaration(
        self, ctx: Modelica.Short_component_declarationContext
    ):
        pass

    # Enter a parse tree produced by Modelica#short_definition.
    def enterShort_definition(self, ctx: Modelica.Short_definitionContext):
        pass

    # Exit a parse tree produced by Modelica#short_definition.
    def exitShort_definition(self, ctx: Modelica.Short_definitionContext):
        pass

    # Enter a parse tree produced by Modelica#equation_section.
    def enterEquation_section(self, ctx: Modelica.Equation_sectionContext):
        pass

    # Exit a parse tree produced by Modelica#equation_section.
    def exitEquation_section(self, ctx: Modelica.Equation_sectionContext):
        pass

    # Enter a parse tree produced by Modelica#algorithm_section.
    def enterAlgorithm_section(self, ctx: Modelica.Algorithm_sectionContext):
        pass

    # Exit a parse tree produced by Modelica#algorithm_section.
    def exitAlgorithm_section(self, ctx: Modelica.Algorithm_sectionContext):
        pass

    # Enter a parse tree produced by Modelica#equation_list.
    def enterEquation_list(self, ctx: Modelica.Equation_listContext):
        pass

    # Exit a parse tree produced by Modelica#equation_list.
    def exitEquation_list(self, ctx: Modelica.Equation_listContext):
        pass

    # Enter a parse tree produced by Modelica#statement_list.
    def enterStatement_list(self, ctx: Modelica.Statement_listContext):
        pass

    # Exit a parse tree produced by Modelica#statement_list.
    def exitStatement_list(self, ctx: Modelica.Statement_listContext):
        pass

    # Enter a parse tree produced by Modelica#equation.
    def enterEquation(self, ctx: Modelica.EquationContext):
        pass

    # Exit a parse tree produced by Modelica#equation.
    def exitEquation(self, ctx: Modelica.EquationContext):
        pass

    # Enter a parse tree produced by Modelica#statement.
    def enterStatement(self, ctx: Modelica.StatementContext):
        pass

    # Exit a parse tree produced by Modelica#statement.
    def exitStatement(self, ctx: Modelica.StatementContext):
        pass

    # Enter a parse tree produced by Modelica#if_equation.
    def enterIf_equation(self, ctx: Modelica.If_equationContext):
        pass

    # Exit a parse tree produced by Modelica#if_equation.
    def exitIf_equation(self, ctx: Modelica.If_equationContext):
        pass

    # Enter a parse tree produced by Modelica#conditional_equations.
    def enterConditional_equations(self, ctx: Modelica.Conditional_equationsContext):
        pass

    # Exit a parse tree produced by Modelica#conditional_equations.
    def exitConditional_equations(self, ctx: Modelica.Conditional_equationsContext):
        pass

    # Enter a parse tree produced by Modelica#if_statement.
    def enterIf_statement(self, ctx: Modelica.If_statementContext):
        pass

    # Exit a parse tree produced by Modelica#if_statement.
    def exitIf_statement(self, ctx: Modelica.If_statementContext):
        pass

    # Enter a parse tree produced by Modelica#if_branch.
    def enterIf_branch(self, ctx: Modelica.If_branchContext):
        pass

    # Exit a parse tree produced by Modelica#if_branch.
    def exitIf_branch(self, ctx: Modelica.If_branchContext):
        pass

    # Enter a parse tree produced by Modelica#elseif_branch.
    def enterElseif_branch(self, ctx: Modelica.Elseif_branchContext):
        pass

    # Exit a parse tree produced by Modelica#elseif_branch.
    def exitElseif_branch(self, ctx: Modelica.Elseif_branchContext):
        pass

    # Enter a parse tree produced by Modelica#else_branch.
    def enterElse_branch(self, ctx: Modelica.Else_branchContext):
        pass

    # Exit a parse tree produced by Modelica#else_branch.
    def exitElse_branch(self, ctx: Modelica.Else_branchContext):
        pass

    # Enter a parse tree produced by Modelica#conditional_statements.
    def enterConditional_statements(self, ctx: Modelica.Conditional_statementsContext):
        pass

    # Exit a parse tree produced by Modelica#conditional_statements.
    def exitConditional_statements(self, ctx: Modelica.Conditional_statementsContext):
        pass

    # Enter a parse tree produced by Modelica#for_equation.
    def enterFor_equation(self, ctx: Modelica.For_equationContext):
        pass

    # Exit a parse tree produced by Modelica#for_equation.
    def exitFor_equation(self, ctx: Modelica.For_equationContext):
        pass

    # Enter a parse tree produced by Modelica#for_statement.
    def enterFor_statement(self, ctx: Modelica.For_statementContext):
        pass

    # Exit a parse tree produced by Modelica#for_statement.
    def exitFor_statement(self, ctx: Modelica.For_statementContext):
        pass

    # Enter a parse tree produced by Modelica#for_indices.
    def enterFor_indices(self, ctx: Modelica.For_indicesContext):
        pass

    # Exit a parse tree produced by Modelica#for_indices.
    def exitFor_indices(self, ctx: Modelica.For_indicesContext):
        pass

    # Enter a parse tree produced by Modelica#for_index.
    def enterFor_index(self, ctx: Modelica.For_indexContext):
        pass

    # Exit a parse tree produced by Modelica#for_index.
    def exitFor_index(self, ctx: Modelica.For_indexContext):
        pass

    # Enter a parse tree produced by Modelica#while_statement.
    def enterWhile_statement(self, ctx: Modelica.While_statementContext):
        pass

    # Exit a parse tree produced by Modelica#while_statement.
    def exitWhile_statement(self, ctx: Modelica.While_statementContext):
        pass

    # Enter a parse tree produced by Modelica#when_equation.
    def enterWhen_equation(self, ctx: Modelica.When_equationContext):
        pass

    # Exit a parse tree produced by Modelica#when_equation.
    def exitWhen_equation(self, ctx: Modelica.When_equationContext):
        pass

    # Enter a parse tree produced by Modelica#when_statement.
    def enterWhen_statement(self, ctx: Modelica.When_statementContext):
        pass

    # Exit a parse tree produced by Modelica#when_statement.
    def exitWhen_statement(self, ctx: Modelica.When_statementContext):
        pass

    # Enter a parse tree produced by Modelica#when_branch.
    def enterWhen_branch(self, ctx: Modelica.When_branchContext):
        pass

    # Exit a parse tree produced by Modelica#when_branch.
    def exitWhen_branch(self, ctx: Modelica.When_branchContext):
        pass

    # Enter a parse tree produced by Modelica#elsewhen_branch.
    def enterElsewhen_branch(self, ctx: Modelica.Elsewhen_branchContext):
        pass

    # Exit a parse tree produced by Modelica#elsewhen_branch.
    def exitElsewhen_branch(self, ctx: Modelica.Elsewhen_branchContext):
        pass

    # Enter a parse tree produced by Modelica#connect_equation.
    def enterConnect_equation(self, ctx: Modelica.Connect_equationContext):
        pass

    # Exit a parse tree produced by Modelica#connect_equation.
    def exitConnect_equation(self, ctx: Modelica.Connect_equationContext):
        pass

    # Enter a parse tree produced by Modelica#connected_components.
    def enterConnected_components(self, ctx: Modelica.Connected_componentsContext):
        pass

    # Exit a parse tree produced by Modelica#connected_components.
    def exitConnected_components(self, ctx: Modelica.Connected_componentsContext):
        pass

    # Enter a parse tree produced by Modelica#expression.
    def enterExpression(self, ctx: Modelica.ExpressionContext):
        pass

    # Exit a parse tree produced by Modelica#expression.
    def exitExpression(self, ctx: Modelica.ExpressionContext):
        pass

    # Enter a parse tree produced by Modelica#if_expression.
    def enterIf_expression(self, ctx: Modelica.If_expressionContext):
        pass

    # Exit a parse tree produced by Modelica#if_expression.
    def exitIf_expression(self, ctx: Modelica.If_expressionContext):
        pass

    # Enter a parse tree produced by Modelica#if_eval.
    def enterIf_eval(self, ctx: Modelica.If_evalContext):
        pass

    # Exit a parse tree produced by Modelica#if_eval.
    def exitIf_eval(self, ctx: Modelica.If_evalContext):
        pass

    # Enter a parse tree produced by Modelica#elseif_eval.
    def enterElseif_eval(self, ctx: Modelica.Elseif_evalContext):
        pass

    # Exit a parse tree produced by Modelica#elseif_eval.
    def exitElseif_eval(self, ctx: Modelica.Elseif_evalContext):
        pass

    # Enter a parse tree produced by Modelica#else_eval.
    def enterElse_eval(self, ctx: Modelica.Else_evalContext):
        pass

    # Exit a parse tree produced by Modelica#else_eval.
    def exitElse_eval(self, ctx: Modelica.Else_evalContext):
        pass

    # Enter a parse tree produced by Modelica#conditional_expression.
    def enterConditional_expression(self, ctx: Modelica.Conditional_expressionContext):
        pass

    # Exit a parse tree produced by Modelica#conditional_expression.
    def exitConditional_expression(self, ctx: Modelica.Conditional_expressionContext):
        pass

    # Enter a parse tree produced by Modelica#simple_expression.
    def enterSimple_expression(self, ctx: Modelica.Simple_expressionContext):
        pass

    # Exit a parse tree produced by Modelica#simple_expression.
    def exitSimple_expression(self, ctx: Modelica.Simple_expressionContext):
        pass

    # Enter a parse tree produced by Modelica#logical_expression.
    def enterLogical_expression(self, ctx: Modelica.Logical_expressionContext):
        pass

    # Exit a parse tree produced by Modelica#logical_expression.
    def exitLogical_expression(self, ctx: Modelica.Logical_expressionContext):
        pass

    # Enter a parse tree produced by Modelica#or_operator.
    def enterOr_operator(self, ctx: Modelica.Or_operatorContext):
        pass

    # Exit a parse tree produced by Modelica#or_operator.
    def exitOr_operator(self, ctx: Modelica.Or_operatorContext):
        pass

    # Enter a parse tree produced by Modelica#logical_term.
    def enterLogical_term(self, ctx: Modelica.Logical_termContext):
        pass

    # Exit a parse tree produced by Modelica#logical_term.
    def exitLogical_term(self, ctx: Modelica.Logical_termContext):
        pass

    # Enter a parse tree produced by Modelica#and_operator.
    def enterAnd_operator(self, ctx: Modelica.And_operatorContext):
        pass

    # Exit a parse tree produced by Modelica#and_operator.
    def exitAnd_operator(self, ctx: Modelica.And_operatorContext):
        pass

    # Enter a parse tree produced by Modelica#logical_factor.
    def enterLogical_factor(self, ctx: Modelica.Logical_factorContext):
        pass

    # Exit a parse tree produced by Modelica#logical_factor.
    def exitLogical_factor(self, ctx: Modelica.Logical_factorContext):
        pass

    # Enter a parse tree produced by Modelica#relation.
    def enterRelation(self, ctx: Modelica.RelationContext):
        pass

    # Exit a parse tree produced by Modelica#relation.
    def exitRelation(self, ctx: Modelica.RelationContext):
        pass

    # Enter a parse tree produced by Modelica#relational_operator.
    def enterRelational_operator(self, ctx: Modelica.Relational_operatorContext):
        pass

    # Exit a parse tree produced by Modelica#relational_operator.
    def exitRelational_operator(self, ctx: Modelica.Relational_operatorContext):
        pass

    # Enter a parse tree produced by Modelica#arithmetic_expression.
    def enterArithmetic_expression(self, ctx: Modelica.Arithmetic_expressionContext):
        pass

    # Exit a parse tree produced by Modelica#arithmetic_expression.
    def exitArithmetic_expression(self, ctx: Modelica.Arithmetic_expressionContext):
        pass

    # Enter a parse tree produced by Modelica#unary_expression.
    def enterUnary_expression(self, ctx: Modelica.Unary_expressionContext):
        pass

    # Exit a parse tree produced by Modelica#unary_expression.
    def exitUnary_expression(self, ctx: Modelica.Unary_expressionContext):
        pass

    # Enter a parse tree produced by Modelica#unary_operand.
    def enterUnary_operand(self, ctx: Modelica.Unary_operandContext):
        pass

    # Exit a parse tree produced by Modelica#unary_operand.
    def exitUnary_operand(self, ctx: Modelica.Unary_operandContext):
        pass

    # Enter a parse tree produced by Modelica#add_operator.
    def enterAdd_operator(self, ctx: Modelica.Add_operatorContext):
        pass

    # Exit a parse tree produced by Modelica#add_operator.
    def exitAdd_operator(self, ctx: Modelica.Add_operatorContext):
        pass

    # Enter a parse tree produced by Modelica#term.
    def enterTerm(self, ctx: Modelica.TermContext):
        pass

    # Exit a parse tree produced by Modelica#term.
    def exitTerm(self, ctx: Modelica.TermContext):
        pass

    # Enter a parse tree produced by Modelica#mul_operator.
    def enterMul_operator(self, ctx: Modelica.Mul_operatorContext):
        pass

    # Exit a parse tree produced by Modelica#mul_operator.
    def exitMul_operator(self, ctx: Modelica.Mul_operatorContext):
        pass

    # Enter a parse tree produced by Modelica#factor.
    def enterFactor(self, ctx: Modelica.FactorContext):
        pass

    # Exit a parse tree produced by Modelica#factor.
    def exitFactor(self, ctx: Modelica.FactorContext):
        pass

    # Enter a parse tree produced by Modelica#exp_operator.
    def enterExp_operator(self, ctx: Modelica.Exp_operatorContext):
        pass

    # Exit a parse tree produced by Modelica#exp_operator.
    def exitExp_operator(self, ctx: Modelica.Exp_operatorContext):
        pass

    # Enter a parse tree produced by Modelica#primary.
    def enterPrimary(self, ctx: Modelica.PrimaryContext):
        pass

    # Exit a parse tree produced by Modelica#primary.
    def exitPrimary(self, ctx: Modelica.PrimaryContext):
        pass

    # Enter a parse tree produced by Modelica#type_specifier.
    def enterType_specifier(self, ctx: Modelica.Type_specifierContext):
        pass

    # Exit a parse tree produced by Modelica#type_specifier.
    def exitType_specifier(self, ctx: Modelica.Type_specifierContext):
        pass

    # Enter a parse tree produced by Modelica#name.
    def enterName(self, ctx: Modelica.NameContext):
        pass

    # Exit a parse tree produced by Modelica#name.
    def exitName(self, ctx: Modelica.NameContext):
        pass

    # Enter a parse tree produced by Modelica#matrix.
    def enterMatrix(self, ctx: Modelica.MatrixContext):
        pass

    # Exit a parse tree produced by Modelica#matrix.
    def exitMatrix(self, ctx: Modelica.MatrixContext):
        pass

    # Enter a parse tree produced by Modelica#matrix_arguments.
    def enterMatrix_arguments(self, ctx: Modelica.Matrix_argumentsContext):
        pass

    # Exit a parse tree produced by Modelica#matrix_arguments.
    def exitMatrix_arguments(self, ctx: Modelica.Matrix_argumentsContext):
        pass

    # Enter a parse tree produced by Modelica#matrix_row.
    def enterMatrix_row(self, ctx: Modelica.Matrix_rowContext):
        pass

    # Exit a parse tree produced by Modelica#matrix_row.
    def exitMatrix_row(self, ctx: Modelica.Matrix_rowContext):
        pass

    # Enter a parse tree produced by Modelica#array.
    def enterArray(self, ctx: Modelica.ArrayContext):
        pass

    # Exit a parse tree produced by Modelica#array.
    def exitArray(self, ctx: Modelica.ArrayContext):
        pass

    # Enter a parse tree produced by Modelica#function_call.
    def enterFunction_call(self, ctx: Modelica.Function_callContext):
        pass

    # Exit a parse tree produced by Modelica#function_call.
    def exitFunction_call(self, ctx: Modelica.Function_callContext):
        pass

    # Enter a parse tree produced by Modelica#component_reference.
    def enterComponent_reference(self, ctx: Modelica.Component_referenceContext):
        pass

    # Exit a parse tree produced by Modelica#component_reference.
    def exitComponent_reference(self, ctx: Modelica.Component_referenceContext):
        pass

    # Enter a parse tree produced by Modelica#function_call_args.
    def enterFunction_call_args(self, ctx: Modelica.Function_call_argsContext):
        pass

    # Exit a parse tree produced by Modelica#function_call_args.
    def exitFunction_call_args(self, ctx: Modelica.Function_call_argsContext):
        pass

    # Enter a parse tree produced by Modelica#function_arguments.
    def enterFunction_arguments(self, ctx: Modelica.Function_argumentsContext):
        pass

    # Exit a parse tree produced by Modelica#function_arguments.
    def exitFunction_arguments(self, ctx: Modelica.Function_argumentsContext):
        pass

    # Enter a parse tree produced by Modelica#function_argument.
    def enterFunction_argument(self, ctx: Modelica.Function_argumentContext):
        pass

    # Exit a parse tree produced by Modelica#function_argument.
    def exitFunction_argument(self, ctx: Modelica.Function_argumentContext):
        pass

    # Enter a parse tree produced by Modelica#function_partial_application.
    def enterFunction_partial_application(
        self, ctx: Modelica.Function_partial_applicationContext
    ):
        pass

    # Exit a parse tree produced by Modelica#function_partial_application.
    def exitFunction_partial_application(
        self, ctx: Modelica.Function_partial_applicationContext
    ):
        pass

    # Enter a parse tree produced by Modelica#named_arguments.
    def enterNamed_arguments(self, ctx: Modelica.Named_argumentsContext):
        pass

    # Exit a parse tree produced by Modelica#named_arguments.
    def exitNamed_arguments(self, ctx: Modelica.Named_argumentsContext):
        pass

    # Enter a parse tree produced by Modelica#named_argument.
    def enterNamed_argument(self, ctx: Modelica.Named_argumentContext):
        pass

    # Exit a parse tree produced by Modelica#named_argument.
    def exitNamed_argument(self, ctx: Modelica.Named_argumentContext):
        pass

    # Enter a parse tree produced by Modelica#output_expression_list.
    def enterOutput_expression_list(self, ctx: Modelica.Output_expression_listContext):
        pass

    # Exit a parse tree produced by Modelica#output_expression_list.
    def exitOutput_expression_list(self, ctx: Modelica.Output_expression_listContext):
        pass

    # Enter a parse tree produced by Modelica#expression_list.
    def enterExpression_list(self, ctx: Modelica.Expression_listContext):
        pass

    # Exit a parse tree produced by Modelica#expression_list.
    def exitExpression_list(self, ctx: Modelica.Expression_listContext):
        pass

    # Enter a parse tree produced by Modelica#expression_list_member.
    def enterExpression_list_member(self, ctx: Modelica.Expression_list_memberContext):
        pass

    # Exit a parse tree produced by Modelica#expression_list_member.
    def exitExpression_list_member(self, ctx: Modelica.Expression_list_memberContext):
        pass

    # Enter a parse tree produced by Modelica#array_arguments.
    def enterArray_arguments(self, ctx: Modelica.Array_argumentsContext):
        pass

    # Exit a parse tree produced by Modelica#array_arguments.
    def exitArray_arguments(self, ctx: Modelica.Array_argumentsContext):
        pass

    # Enter a parse tree produced by Modelica#for_initializer.
    def enterFor_initializer(self, ctx: Modelica.For_initializerContext):
        pass

    # Exit a parse tree produced by Modelica#for_initializer.
    def exitFor_initializer(self, ctx: Modelica.For_initializerContext):
        pass

    # Enter a parse tree produced by Modelica#array_argument.
    def enterArray_argument(self, ctx: Modelica.Array_argumentContext):
        pass

    # Exit a parse tree produced by Modelica#array_argument.
    def exitArray_argument(self, ctx: Modelica.Array_argumentContext):
        pass

    # Enter a parse tree produced by Modelica#array_subscripts.
    def enterArray_subscripts(self, ctx: Modelica.Array_subscriptsContext):
        pass

    # Exit a parse tree produced by Modelica#array_subscripts.
    def exitArray_subscripts(self, ctx: Modelica.Array_subscriptsContext):
        pass

    # Enter a parse tree produced by Modelica#subscript_list.
    def enterSubscript_list(self, ctx: Modelica.Subscript_listContext):
        pass

    # Exit a parse tree produced by Modelica#subscript_list.
    def exitSubscript_list(self, ctx: Modelica.Subscript_listContext):
        pass

    # Enter a parse tree produced by Modelica#subscript.
    def enterSubscript(self, ctx: Modelica.SubscriptContext):
        pass

    # Exit a parse tree produced by Modelica#subscript.
    def exitSubscript(self, ctx: Modelica.SubscriptContext):
        pass

    # Enter a parse tree produced by Modelica#description.
    def enterDescription(self, ctx: Modelica.DescriptionContext):
        pass

    # Exit a parse tree produced by Modelica#description.
    def exitDescription(self, ctx: Modelica.DescriptionContext):
        pass

    # Enter a parse tree produced by Modelica#description_string.
    def enterDescription_string(self, ctx: Modelica.Description_stringContext):
        pass

    # Exit a parse tree produced by Modelica#description_string.
    def exitDescription_string(self, ctx: Modelica.Description_stringContext):
        pass

    # Enter a parse tree produced by Modelica#cat_operator.
    def enterCat_operator(self, ctx: Modelica.Cat_operatorContext):
        pass

    # Exit a parse tree produced by Modelica#cat_operator.
    def exitCat_operator(self, ctx: Modelica.Cat_operatorContext):
        pass

    # Enter a parse tree produced by Modelica#annotation.
    def enterAnnotation(self, ctx: Modelica.AnnotationContext):
        pass

    # Exit a parse tree produced by Modelica#annotation.
    def exitAnnotation(self, ctx: Modelica.AnnotationContext):
        pass


del Modelica
