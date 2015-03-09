<%!
    from util import (put_and, rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment,
                      rb_type, mb_type, singular, hub_type, to_fqan, indent_all_but_first_by,
                      method_params, activity_rust_type, mangle_ident, activity_input_type, get_word,
                      split_camelcase_s, property, is_pod_property, TREF, IO_REQUEST, 
                      schema_to_required_property, rust_copy_value_s, is_required_property,
                      hide_rust_doc_test, build_all_params, REQUEST_VALUE_PROPERTY_NAME, organize_params, 
                      indent_by, to_rust_type, rnd_arg_val_for_type, extract_parts, mb_type_params_s,
                      hub_type_params_s, method_media_params, enclose_in, mb_type_bounds)

    def get_parts(part_prop):
        if not part_prop:
            return list()
        return extract_parts(part_prop.get('description', ''))

    def make_parts_desc(part_prop):
        
        parts = get_parts(part_prop)
        if not parts:
            return None
        part_desc = "**Settable Parts**\n\n"
        part_desc += ''.join('* *%s*\n' % part for part in parts)
        part_desc = part_desc[:-1]
        return part_desc
%>\
<%namespace name="util" file="util.mako"/>\
<%namespace name="lib" file="lib.mako"/>\

## Creates a method builder type
###############################################################################################
###############################################################################################
<%def name="new(resource, method, c)">\
<% 
    hub_type_name = hub_type(util.canonical_name())
    m = c.fqan_map[to_fqan(name, resource, method)]
    # an identifier for a property. We prefix them to prevent clashes with the setters
    mb_tparams = mb_type_params_s(m)
    ThisType = mb_type(resource, method) + mb_tparams

    params, request_value = build_all_params(schemas, c, m, IO_REQUEST, REQUEST_VALUE_PROPERTY_NAME)
    part_prop = None
    for p in params:
        if p.name == 'part':
            part_prop = p
            break
    # end for each param
    part_desc = make_parts_desc(part_prop)
    parts = get_parts(part_prop)
%>\
% if 'description' in m:
${m.description | rust_doc_comment}
///
% endif
/// A builder for the *${method}* method supported by a *${singular(resource)}* resource.
/// It is not used directly, but through a `${rb_type(resource)}`.
///
% if part_desc:
${part_desc | rust_doc_comment}
///
% if m.get('scopes'):
/// # Scopes
///
/// You will need authorization for \
% if len(m.scopes) > 1:
at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
% for s in m.scopes:
/// * *${s}*
% endfor
% else:
the *${m.scopes[0]}* scope to make a valid call.
% endif
% endif
///
% endif
/// # Example
///
/// Instantiate a resource method builder
///
<%block filter="rust_doc_comment">\
${self.usage(resource, method, m, params, request_value, parts)}\
</%block>
pub struct ${ThisType}
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a ${hub_type_name}${hub_type_params_s()},
## PROPERTIES ###############
% for p in params:
    ${property(p.name)}:\
    % if is_required_property(p):
 ${activity_rust_type(p, allow_optionals=False)},
    % else:
 ${activity_rust_type(p)},
    % endif
% endfor
## A generic map for additinal parameters. Sometimes you can set some that are documented online only
    _additional_params: HashMap<String, String>
}

impl${mb_tparams} MethodBuilder for ${ThisType} {}

impl${mb_tparams} ${ThisType} where ${', '.join(mb_type_bounds())} {

${self._action_fn(resource, method, m, params, request_value, parts)}\

## SETTERS ###############
% for p in params:
${self._setter_fn(resource, method, m, p, part_prop, ThisType, c)}\
% endfor

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    pub fn param(mut self, name: &str, value: &str) -> ${ThisType} {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}
</%def>


## creates a setter for the call builder
###############################################################################################
###############################################################################################
<%def name="_setter_fn(resource, method, m, p, part_prop, ThisType, c)">\
<%
    InType = activity_input_type(p)

    def show_part_info(m, p):
        if p.name != 'part':
            return False
        if not (m.get('request') and m.get('response')):
            return False
        return m.request.get(TREF, 'first') == m.response.get(TREF, 'second')

    value_name = 'new_value'
    new_value_copied = rust_copy_value_s(value_name, InType, p)
    if not is_required_property(p):
        new_value_copied = 'Some(%s)' % new_value_copied

    part_desc = None
    if part_prop is not None and p.name in ('part', REQUEST_VALUE_PROPERTY_NAME):
        part_desc = make_parts_desc(part_prop)
    # end part description
%>\
    /// Sets the *${split_camelcase_s(p.name)}* ${get_word(p, 'location')}property to the given value.
    ///
    % if show_part_info(m, p):
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    % elif is_required_property(p):
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    % endif
    % if part_desc:
    ///
    ${part_desc | rust_doc_comment, indent_all_but_first_by(1)}
    % endif
    /// 
    % if 'description' in p:
    ${p.description | rust_doc_comment, indent_all_but_first_by(1)}
    % endif
    pub fn ${mangle_ident(p.name)}(mut self, ${value_name}: ${InType}) -> ${ThisType} {
        self.${property(p.name)} = ${new_value_copied};
        self
    }
</%def>


## creates usage docs the method builder
###############################################################################################
###############################################################################################
<%def name="usage(resource, method, m, params, request_value, parts)">\
<%
    hub_type_name = hub_type(util.canonical_name())
    required_props, optional_props, part_prop = organize_params(params, request_value)
    is_string_value = lambda v: v.endswith('"')

    # to rust value
    trv = lambda spn, sp, sn=None: to_rust_type(sn, spn, sp, allow_optionals=False)
    # rvfrt = random value for rust type
    rvfrt = lambda spn, sp, sn=None: rnd_arg_val_for_type(trv(spn, sp, sn))

    rb_name = 'req'   # name of request binding
    required_args = request_value and ['&' + rb_name] or []
    for p in required_props:
        # could also just skip the first element, but ... let's be safe
        if request_value and request_value.id == p.get(TREF):
            continue
        v = rvfrt(p.name, p)
        # we chose to replace random strings with their meaning, as indicated by the name !
        if is_string_value(v):
            v = '"%s"' % p.name
        required_args.append(v)
    # end for each required property
    required_args = ', '.join(required_args)

    media_params = method_media_params(m)

    action_name = media_params and (api.terms.upload_action + media_params[-1].type.suffix) or api.terms.action
    action_args = media_params and media_params[-1].type.example_value or ''

    random_value_warning = "Values shown here are possibly random and not representative !"
%>\
<%block filter="rust_doc_test_norun">\
${capture(util.test_prelude) | hide_rust_doc_test}\
% if request_value:
# use ${util.library_name()}::${request_value.id};
% endif
% if media_params:
# use std::fs;
% endif
<%block filter="rust_test_fn_invisible">\
${capture(lib.test_hub, hub_type_name, comments=False) | hide_rust_doc_test}
% if request_value:
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// ${random_value_warning}
let mut ${rb_name}: ${request_value.id} = Default::default();
% for spn, sp in request_value.get('properties', dict()).iteritems():
% if spn not in parts:
<% continue %>
% endif
<%
    rtn = trv(spn, sp, request_value.id)
    assignment = rnd_arg_val_for_type(rtn)
    if is_string_value(assignment):
        assignment = assignment + '.to_string()'
    if assignment.endswith('default()'):
        assignment = assignment[1:] # cut & - it's not ok in this case :)!
        assignment += '; // is %s' % rtn
    else:
        assignment = 'Some(%s);' % assignment
%>\
## ${to_rust_type(request_value.id, spn, sp, allow_optionals=False)}
${rb_name}.${mangle_ident(spn)} = ${assignment}
% endfor

% endif
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `${action_name}(${action_args and '...' or ''})`.
% if optional_props:
// ${random_value_warning}
% endif
let result = hub.${mangle_ident(resource)}().${mangle_ident(method)}(${required_args})\
% for p in optional_props:
% if p.get('skip_example', False):
<% continue %>
% endif

<%block  filter="indent_by(13)">\
.${mangle_ident(p.name)}(${rvfrt(p.name, p)})\
</%block>\
% endfor

${'.' + action_name | indent_by(13)}(${action_args});
// TODO: show how to handle the result !
</%block>
</%block>\
</%def>


## create an entire 'api.terms.action' method
###############################################################################################
###############################################################################################
<%def name="_action_fn(resource, method, m, params, request_value, parts)">\
<%
    media_params = method_media_params(m)

    type_params = ''
    where = ''
    qualifier = 'pub '
    add_args = ''
    rtype = '()'

    if media_params:
        stripped = lambda s: s.strip().strip(',')
        qualifier = ''
        for p in media_params:
            type_params += p.type.param + ', '
            where += p.type.param + ': ' + p.type.where + ', '
            add_args += p.type.arg_name + ': ' + ('Option<(%s, u64, mime::Mime)>' % p.type.param) + ', '
        # end for each param
        where = ' where ' + stripped(where)
        type_params = '<' + stripped(type_params) + '>'
        add_args = ', ' + stripped(add_args)
    # end handle media params

    action_fn = qualifier + 'fn ' + api.terms.action + type_params + ('(mut self%s)' % add_args) + ' -> ' + rtype + where

    field_params = [p for p in params if p.get('is_query_param', True)]
%>
    /// Perform the operation you have build so far.
    ${action_fn} {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(${len(params)});
        % for p in field_params:
<%
    pname = 'self.' + property(p.name)    # property identifier
%>\
        ## parts can also be derived from the request, but we do that only if it's not set
        % if p.name == 'part' and request_value:
        % if not is_required_property(p):
        if ${pname}.is_none() {
            ${pname} = Some(self.${property(REQUEST_VALUE_PROPERTY_NAME)}.to_parts());
        }
        % else:
        if ${pname}.len() == 0 {
            ${pname} = self.${property(REQUEST_VALUE_PROPERTY_NAME)}.to_parts();
        }
        % endif
        % endif
        % if not is_required_property(p):
        if ${pname}.is_some() {
            params.push(("${p.name}", ${pname}.unwrap().to_string()));
        }
        % else:
        params.push(("${p.name}", ${pname}.to_string()));
        % endif
        % endfor
        ## Additional params - may not overlap with optional params
        ## let mut params: Vec<(String, String)> = Vec::with_capacity
        ## // note: cloned() shouldn't be needed, see issue
        ## // https://github.com/servo/rust-url/issues/81
        ## let req = form_urlencoded::serialize(
        ##           [("client_id", client_id),
        ##            ("scope", scopes.into_iter()
        ##                            .map(|s| s.as_slice())
        ##                            .intersperse(" ")
        ##                            .collect::<String>()
        ##                            .as_slice())].iter().cloned());

        ## match self.client.borrow_mut().post(FlowType::Device.as_slice())
        ##        .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
        ##        .body(req.as_slice())
        ##        .send() {
        ##     Err(err) => {
        ##         return RequestResult::Error(err);
        ##     }
    }

    % for p in media_params:
<% 
        none_type = 'None::<(' + p.type.default + ', u64, mime::Mime)>' 
%>\
    ${p.description | rust_doc_comment, indent_all_but_first_by(1)}
    ///
    % for item_name, item in p.info.iteritems():
    /// * *${split_camelcase_s(item_name)}*: ${isinstance(item, (list, tuple)) and put_and(enclose_in("'", item)) or str(item)}
    % endfor
    pub fn ${api.terms.upload_action}${p.type.suffix}<${p.type.param}>(mut self, ${p.type.arg_name}: ${p.type.param}, size: u64, mime_type: mime::Mime) -> ${rtype}
                where ${p.type.param}: ${p.type.where} {
        self.${api.terms.action}(\
        % for _ in range(0, loop.index):
${none_type}, \
        % endfor
Some((${p.type.arg_name}, size, mime_type)), \
        % for _ in range(loop.index+1, len(media_params)):
${none_type}, \
        % endfor
)
    }
    % endfor
</%def>