use leptos::*;
use serde_json::Value;
use strum::IntoEnumIterator;
use superposition_types::database::models::others::{
    CustomHeaders, HttpMethod, PayloadVersion, WebhookEvent,
};
use web_sys::MouseEvent;

use crate::{
    api::{create_webhook, update_webhook},
    components::{
        alert::AlertType,
        button::Button,
        change_form::ChangeForm,
        dropdown::{Dropdown, DropdownBtnType, DropdownDirection},
        form::label::Label,
        input::{Input, InputType, Toggle},
    },
    providers::{alert_provider::enqueue_alert, editor_provider::EditorProvider},
    schema::{JsonSchemaType, SchemaType::Single},
    types::{OrganisationId, Tenant},
};

#[component]
pub fn webhook_form<NF>(
    #[prop(default = false)] edit: bool,
    #[prop(default = String::new())] webhook_name: String,
    #[prop(default = String::new())] description: String,
    #[prop(default = false)] enabled: bool,
    #[prop(default = String::new())] url: String,
    #[prop(default = HttpMethod::default())] method: HttpMethod,
    #[prop(default = PayloadVersion::default())] payload_version: PayloadVersion,
    #[prop(default = CustomHeaders::default())] custom_headers: CustomHeaders,
    #[prop(default = Vec::new())] events: Vec<WebhookEvent>,
    handle_submit: NF,
) -> impl IntoView
where
    NF: Fn() + 'static + Clone,
{
    let workspace = use_context::<Signal<Tenant>>().unwrap();
    let org = use_context::<Signal<OrganisationId>>().unwrap();

    let (webhook_name_rs, webhook_name_ws) = create_signal(webhook_name);
    let (description_rs, description_ws) = create_signal(description);
    let (enabled_rs, enabled_ws) = create_signal(enabled);
    let (url_rs, url_ws) = create_signal(url);
    let (method_rs, method_ws) = create_signal(method);
    let (payload_version_rs, payload_version_ws) = create_signal(payload_version);
    let (custom_headers_rs, custom_headers_ws) = create_signal(custom_headers);
    let (change_reason_rs, change_reason_ws) = create_signal(String::new());
    let (events_rs, events_ws) = create_signal(events);
    let (req_inprogess_rs, req_inprogress_ws) = create_signal(false);

    let handle_select_webhook_event_dropdown_option =
        Callback::new(move |selected_event: WebhookEvent| {
            events_ws.update(|value| value.push(selected_event));
        });

    let handle_remove_webhook_event_dropdown_option =
        Callback::new(move |selected_event: WebhookEvent| {
            events_ws.update(|value| value.retain(|d| d != &selected_event));
        });

    let on_submit = move |ev: MouseEvent| {
        req_inprogress_ws.set(true);
        ev.prevent_default();

        let webhook_name = webhook_name_rs.get();
        let description = description_rs.get();
        let enabled = enabled_rs.get();
        let url = url_rs.get();
        let method = method_rs.get();
        let payload_version = payload_version_rs.get();
        let custom_headers = custom_headers_rs.get();
        let change_reason = change_reason_rs.get();
        let events = events_rs.get();

        let handle_submit_clone = handle_submit.clone();
        spawn_local({
            let handle_submit = handle_submit_clone;

            async move {
                let result = if edit {
                    update_webhook(
                        webhook_name,
                        enabled,
                        url,
                        method,
                        payload_version,
                        custom_headers,
                        events,
                        description,
                        change_reason,
                        workspace.get().0,
                        org.get().0,
                    )
                    .await
                } else {
                    create_webhook(
                        webhook_name,
                        description,
                        enabled,
                        url,
                        method,
                        payload_version,
                        custom_headers,
                        events,
                        change_reason,
                        workspace.get().0,
                        org.get().0,
                    )
                    .await
                };

                req_inprogress_ws.set(false);
                match result {
                    Ok(_) => {
                        handle_submit();
                        let success_message = if edit {
                            "Webhook updated successfully!"
                        } else {
                            "New Webhook created successfully!"
                        };
                        enqueue_alert(
                            String::from(success_message),
                            AlertType::Success,
                            5000,
                        );
                    }
                    Err(e) => {
                        enqueue_alert(e, AlertType::Error, 5000);
                    }
                }
            }
        });
    };

    let method_options = HttpMethod::iter().collect::<Vec<HttpMethod>>();
    let payload_version_options = PayloadVersion::iter().collect::<Vec<PayloadVersion>>();
    let events_options = WebhookEvent::iter().collect::<Vec<WebhookEvent>>();

    view! {
        <form class="w-full flex flex-col gap-5 bg-white text-gray-700">
            <div class="form-control">
                <Label
                    title="Webhook Name"
                    description="The name of the webhook. It should be unique within the workspace."
                />
                <Input
                    disabled=edit
                    r#type=InputType::Text
                    placeholder="Webhook Name"
                    class="input input-bordered w-full max-w-md"
                    value=Value::String(webhook_name_rs.get())
                    schema_type=Single(JsonSchemaType::String)
                    on_change=Callback::new(move |value: Value| {
                        webhook_name_ws.set(value.to_string().replace('"', ""));
                    })
                />
            </div>

            <ChangeForm
                title="Description".to_string()
                placeholder="Enter a description".to_string()
                value=description_rs.get_untracked()
                on_change=move |new_description| description_ws.set(new_description)
            />
            <ChangeForm
                title="Reason for Change".to_string()
                placeholder="Enter a reason for this change".to_string()
                value=change_reason_rs.get_untracked()
                on_change=move |new_change_reason| change_reason_ws.set(new_change_reason)
            />
            <div class="w-fit flex items-center gap-2">
                <Toggle
                    name="Enable Webhook"
                    value=enabled_rs.get()
                    on_change=move |_| enabled_ws.update(|v| *v = !*v)
                />
                <Label title="Enable Webhook" />
            </div>

            <div class="form-control">
                <Label title="URL" description="The URL to which the webhook will send requests." />
                <Input
                    placeholder="Enter the webhook URL"
                    class="textarea textarea-bordered w-full max-w-md"
                    value=Value::String(url_rs.get_untracked())
                    schema_type=Single(JsonSchemaType::String)
                    r#type=InputType::Text
                    on_change=Callback::new(move |value: Value| {
                        url_ws.set(value.to_string().replace('"', ""));
                    })
                />
            </div>

            <div class="form-control">
                <Label
                    title="Method"
                    description="HTTP method to be used for the webhook request."
                />
                <Dropdown
                    dropdown_width="w-100"
                    dropdown_icon="".to_string()
                    dropdown_text=method_rs.get_untracked().to_string()
                    dropdown_direction=DropdownDirection::Down
                    dropdown_btn_type=DropdownBtnType::Select
                    dropdown_options=method_options
                    on_select=Callback::new(move |selected_item: HttpMethod| {
                        logging::log!("selected item {:?}", selected_item);
                        method_ws.set(selected_item);
                    })
                />
            </div>

            <div class="form-control">
                <Label title="Paylaod Version" />
                <Dropdown
                    dropdown_width="w-100"
                    dropdown_icon="".to_string()
                    dropdown_text=payload_version_rs.get().to_string()
                    dropdown_direction=DropdownDirection::Down
                    dropdown_btn_type=DropdownBtnType::Select
                    dropdown_options=payload_version_options
                    on_select=Callback::new(move |selected_item: PayloadVersion| {
                        logging::log!("selected item {:?}", selected_item);
                        payload_version_ws.set(selected_item);
                    })
                />
            </div>

            <div class="form-control">
                <Label
                    title="Events"
                    description="Events for which this webhook will be triggered."
                />
                <Dropdown
                    dropdown_text="Add Events".to_string()
                    dropdown_direction=DropdownDirection::Down
                    dropdown_btn_type=DropdownBtnType::Select
                    dropdown_options=events_options
                    selected=events_rs.get()
                    multi_select=true
                    on_select=handle_select_webhook_event_dropdown_option
                    on_remove=handle_remove_webhook_event_dropdown_option
                />
            </div>

            <div class="form-control">
                <Label
                    title="Custom Headers"
                    description="Custom headers are optional and can be used to pass additional information with the webhook request."
                />
                <EditorProvider>
                    <Input
                        id="custom_headers"
                        class="rounded-md resize-y w-full max-w-md"
                        value=Value::Object((*custom_headers_rs.get_untracked()).clone())
                        schema_type=Single(JsonSchemaType::Object)
                        r#type=InputType::Monaco(vec![])
                        on_change=move |value: Value| {
                            if let Some(val) = value.as_object() {
                                custom_headers_ws.set(CustomHeaders::from(val.clone()));
                            }
                        }
                    />
                </EditorProvider>
            </div>

            {move || {
                let loading = req_inprogess_rs.get();
                view! {
                    <Button
                        class="self-end h-12 w-48"
                        text="Submit"
                        icon_class="ri-send-plane-line"
                        on_click=on_submit.clone()
                        loading
                    />
                }
            }}

        </form>
    }
}
