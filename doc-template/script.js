

const map_data = {
    "/sample.html": "data/sample"
}

let path_data;
let component_data;
const path = window.location.pathname;

// define the path data
fetch(map_data[path] + "_data.json")
    .then(response => response.json())
    .then(response => {
        path_data = response
    })
    .catch(err => console.log('Failed to load json:', err))

// load component data
fetch(map_data[path] + "_component.json")
    .then(response => response.json())
    .then(response => {
        component_data = response
    })
    .catch(err => console.log('Failed to load json:', err))



const  schema_ref = (ref) => {
  // Remove the leading '#' and split the path
  const path = ref.replace(/^#\/components/, '').split('/').filter(segment => segment);
  
  // Traverse the schema object
  let current = component_data;
  for (const segment of path) {
    if (current[segment] === undefined) {
      throw new Error(`Reference ${ref} not found in schema`);
    }
    current = current[segment];
  }
  return current;
}


const set_description = (h3, id, method, code) => {
    // console.log(`${id}:${method}:${code}`)
    h3.textContent = path_data?.paths?.[id]?.[method].responses?.[code]?.description || "{NULL}";
}

const set_content_type = (content, id, method, code) => {
    let content_obj = path_data?.paths?.[id]?.[method].responses?.[code]?.content;
    let type = Object.keys(content_obj)[0];
    content.textContent = type;
    return type
}


function schemaToExample(schema) {
    // Follow $ref if present
    if (schema.$ref) {
        const resolvedSchema = schema_ref(schema.$ref);
        return schemaToExample(resolvedSchema);
    }

    if (schema.type === 'object') {
        const example = {};
        const props = schema.properties || {};
        for (const key in props) {
            example[key] = schemaToExample(props[key]);
        }
        return example;
    }

    if (schema.type === 'array') {
        return [schemaToExample(schema.items)];
    }

    // Primitive fallback: string, integer, boolean, etc.
    if (schema.example !== undefined) return schema.example;

    switch (schema.type) {
        case 'string': return 'string';
        case 'integer': return 0;
        case 'number': return 0.0;
        case 'boolean': return true;
        default: return schema.type ?? 'unknown';
    }
}


function componentToTypeJson(schema) {
    if (schema.$ref) {
        const resolvedSchema = schema_ref(schema.$ref);
        return componentToTypeJson(resolvedSchema);
    }

    if (schema.type === 'object') {
        const result = {};
        const props = schema.properties || {};
        for (const key in props) {
            result[key] = componentToTypeJson(props[key]);
        }
        return result;
    }

    if (schema.type === 'array') {
        return [componentToTypeJson(schema.items)];
    }

    return schema.type ?? 'unknown';
}

const set_code_display_response = (
    code_container, 
    id, 
    method, 
    code,
    type
) => {
    let ref = path_data?.paths?.[id]?.[method]
        .responses?.[code]?.content?.[type].schema.$ref;


    let thingy = schema_ref(ref);
    let json_obj = componentToTypeJson(thingy)
    code_container.textContent = JSON.stringify(json_obj, null, 2);
    if (window.hljs && window.hljs.highlightBlock) {
        window.hljs.highlightBlock(code_container);
    } else {
        console.warn("Highlight.js (v9) is not loaded properly.");
    }
}


const set_code_display_request = (
    code_block, 
    id, 
    method, 
    type,
    display
) => {

    
    let ref = display == "schema"? ( 
        componentToTypeJson(schema_ref(
            path_data?.paths?.[id]?.[method]
            .requestBody?.content?.[type].schema.$ref
        ))
    ) : (
        path_data?.paths?.[id]?.[method]
            .requestBody?.content?.[type]?.[display]
    );


    code_block.textContent = JSON.stringify(ref, null, 2);
    if (window.hljs && window.hljs.highlightBlock) {
        window.hljs.highlightBlock(code_block);
    } else {
        console.warn("Highlight.js (v9) is not loaded properly.");
    }
}



function response_detail(this_document, code) {
    
    //set description
    let container = this_document.parentElement.parentElement;
    const target_id = container.id;
    const method = container.getAttribute('method');

    
    if (container && target_id) {
        //define title
        let desc = container.querySelector('h4');
        set_description(desc, target_id, method, code);

        // set content type
        let cont_type = container.querySelectorAll('pre.content-type p')[1];
        let content_type = set_content_type(cont_type, target_id, method, code)

        // 
        let code_container = container.querySelector('pre code');
        set_code_display_response(
            code_container, 
            target_id, 
            method, 
            code,
            content_type, 
        );


    } else {
        console.error(`Cant find response body: ${target_id}:${method}:${code}`)
    }

}



// -----------------------------------
//
//  RESPONSE BOX FUNCTIONS
//
// -----------------------------------

function setActiveTab(tabElement) {
  // Remove .selected from all
    let display_option = tabElement.parentElement; 
    let container_display = display_option.parentElement;
    let code_block = container_display
        .querySelector('pre code');

    const id = container_display.id;
    const method = container_display.getAttribute('method');
    const type = container_display.getAttribute('type');
    const selected = tabElement.getAttribute('data-tab');
    
    display_option.querySelectorAll('.option-tab')
        .forEach(el => el.classList.remove('selected'));

        
    set_code_display_request(
        code_block,
        id,
        method,
        type,
        selected
    )

    // Add .selected to clicked
    tabElement.classList.add('selected');

    // Move underline
    const underline = display_option.querySelector('.underline');
    underline.style.width = `${tabElement.offsetWidth}px`;
    underline.style.left = `${tabElement.offsetLeft}px`;


}

// Initialize on page load
window.addEventListener('DOMContentLoaded', () => {
  // Get all containers that contain option-tabs and underlines
  const containers = document.querySelectorAll('.option-display');

  containers.forEach(container => {
    const selected = container.querySelector('.option-tab.selected');
    const underline = container.querySelector('.underline');

    if (selected && underline) {
      underline.style.width = `${selected.offsetWidth}px`;
      underline.style.left = `${selected.offsetLeft}px`;
    }
  });
});

// -----------------------------------------------





// -----------------------------------
//
//  DROPDOWN TAGS FUNCTIONS
//
// -----------------------------------

function toggleDropdown(header) {
    const dropdown = header.parentElement;
    const content = dropdown.querySelector('.dropdown-content');
    const items = content.querySelectorAll('.dropdown-item');
    const itemCount = items.length;
    
    
    // Calculate dynamic timing based on item count
    const baseTime = 300; // Base time in ms
    const itemTime = 30; // Additional time per item
    const maxTime = 600; // Maximum time cap
    const dynamicTime = Math.min(baseTime + (itemCount * itemTime), maxTime);
    
    // Close other dropdowns first
    const allDropdowns = document.querySelectorAll('.dropdown');
    allDropdowns.forEach(otherDropdown => {
        if (otherDropdown !== dropdown && otherDropdown.classList.contains('active')) {
            const otherContent = otherDropdown.querySelector('.dropdown-content');
            const otherItems = otherContent.querySelectorAll('.dropdown-item');
            const otherDynamicTime = Math.min(baseTime + (otherItems.length * itemTime), maxTime);
            
            otherDropdown.classList.remove('active');
            otherContent.style.transition = `max-height ${otherDynamicTime}ms cubic-bezier(0.4, 0, 0.2, 1), padding ${otherDynamicTime}ms cubic-bezier(0.4, 0, 0.2, 1)`;
            otherContent.style.maxHeight = '0px';
        }
    });
    
    // Toggle current dropdown
    const isActive = dropdown.classList.contains('active');
    
    if (isActive) {
        // Closing
        dropdown.classList.remove('active');
        content.style.transition = `max-height ${dynamicTime}ms cubic-bezier(0.4, 0, 0.2, 1), padding ${dynamicTime}ms cubic-bezier(0.4, 0, 0.2, 1)`;
        content.style.maxHeight = '0px';
    } else {
        // Opening
        dropdown.classList.add('active');
        
        // Calculate content height
        const contentHeight = Array.from(items).reduce((total, item) => {
            return total + item.offsetHeight;
        }, 0) + 16; // 16px for padding
        
        content.style.transition = `max-height ${dynamicTime}ms cubic-bezier(0.4, 0, 0.2, 1), padding ${dynamicTime}ms cubic-bezier(0.4, 0, 0.2, 1)`;
        content.style.maxHeight = contentHeight + 'px';
    }
}





// Clicking outside the dropdown closes the active dropdown
document.addEventListener('click', function(event) {
    if (!event.target.closest('.dropdown-container')) {
        // Calculate dynamic timing based on item count
        const itemTime = 30; // Additional time per item
        const maxTime = 600; // Maximum time cap
        const baseTime = 300; // Base time in ms

        const allDropdowns = document.querySelectorAll('.dropdown');
        allDropdowns.forEach(otherDropdown => {
            if (otherDropdown.classList.contains('active')) {
                const otherContent = otherDropdown.querySelector('.dropdown-content');
                const otherItems = otherContent.querySelectorAll('.dropdown-item');
                const otherDynamicTime = Math.min(baseTime + (otherItems.length * itemTime), maxTime);
                
                otherDropdown.classList.remove('active');
                otherContent.style.transition = `max-height ${otherDynamicTime}ms cubic-bezier(0.4, 0, 0.2, 1), padding ${otherDynamicTime}ms cubic-bezier(0.4, 0, 0.2, 1)`;
                otherContent.style.maxHeight = '0px';
            }
        });
    }
});