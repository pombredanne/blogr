
var resizeInput = document.getElementById("inputTitle");

// https://stackoverflow.com/a/3395975/7891095
if (resizeInput) {
    adjustWidthOfInput();
    resizeInput.onkeyup = adjustWidthOfInput;
}

function getWidthOfInput() {
    if (!resizeInput) return;
    
    var tmp = document.createElement("span");
    
    tmp.className = "input-element tmp-element";
    tmp.innerHTML = resizeInput.value.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
    
    tmp.style.visibility = 'hidden';
    tmp.style.fontFamily = resizeInput.style.fontFamily;
    tmp.style.fontSize = resizeInput.style.fontSize;
    tmp.style.letterSpacing = resizeInput.style.letterSpacing;
    tmp.style.padding = resizeInput.style.padding;
    tmp.style.paddingLeft = resizeInput.style.paddingLeft;
    tmp.style.paddingRight = resizeInput.style.paddingRight;
    tmp.style.paddingTop = resizeInput.style.paddingTop;
    tmp.style.paddingBottom = resizeInput.style.paddingBottom;
    
    document.body.appendChild(tmp);
    var theWidth = tmp.getBoundingClientRect().width;
    document.body.removeChild(tmp);
    
    return theWidth;
}

function adjustWidthOfInput() {
    if (!resizeInput) return;
    var w = getWidthOfInput();
    var extra = 180;
    var min = 200;
    var max = 735;
    var width;
    // if (w < 300) {
        // width = 300;
    // } else {
        // width = w + 150;
    // }
    
    if (w < min) {
        width = min;
    } else if (w+extra > max) {
        width = max-extra;
    } else {
        width = w;
    }
    
    resizeInput.style.width = width + extra + "px";
    // resizeInput.style.width = getWidthOfInput() + "px";
}




// var searchform = document.getElementById('search-form').element["q"].style.display = 'none';
var searchfield = document.getElementById('search-form').q;
var searchbtn = document.getElementById('search-form').lastElementChild;
if (searchfield && searchbtn) {
    hide_search();
    searchfield.addEventListener('change', function() { show_search(); });
    searchfield.addEventListener('focus', function() { show_search(); });
    searchfield.addEventListener('blur', function() { hide_search(); });
} else {
    console.log("No search field or button");
}

function hide_search() {
    // commented out code to make the search button stay visible if
    //     there is text in the search field
    // if (searchfield.value == "") {
        searchbtn.style.display = 'none';
    // } else {
        // searchbtn.style.display = 'block';
    // }
}

function show_search() {
    searchbtn.style.display = 'block';
}


// var tag_form = document.forms.insert_form.elements["tags"];
var tag_form = document.getElementById("insert-tags");
if (tag_form) {
    tag_form.addEventListener('input', function(event) {
        checkTags();
    }, false);
    tag_form.addEventListener('change', function(event) {
        checkTags();
    }, false);
    tag_form.addEventListener('mouseout', function(event) {
        checkTags();
    }, false);
}

var manage = document.getElementById('v-am-list');
if (manage) {
    
    $(function () {
        $('[data-toggle="tooltip"]').tooltip()
    })
}


var prev_start = document.getElementById('preview-edit-start');
var prev_start2 = document.getElementById('preview-edit-start2');
var prev_end = document.getElementById('preview-edit-end');
var prev_end2 = document.getElementById('preview-edit-end2');

if (prev_start && prev_end) {
    prev_start.addEventListener('click', function() {
        preview_edit();
    });
    prev_start.addEventListener('keydown', function () {
        if (event.keyCode === 13) {
            preview_edit();
        }
    });
    prev_end.addEventListener('click', function () {
        preview_edit_end();
    });
    prev_end.addEventListener('keydown', function () {
        if (event.keyCode === 13) {
            preview_edit_end();
        }
    });
}
if (prev_start2) {
    prev_start2.addEventListener('click', function() {
        preview_edit();
    });
    prev_start2.addEventListener('keydown', function () {
        if (event.keyCode === 13) {
            preview_edit();
        }
    });
}
if (prev_end2) {
    prev_end2.addEventListener('click', function () {
        preview_edit_end();
    });
    prev_end2.addEventListener('keydown', function () {
        if (event.keyCode === 13) {
            preview_edit_end();
        }
    });
}

var formins = document.getElementById('insert_form');
var prev_save = document.getElementById('preview-save');
var prev_save2 = document.getElementById('preview-save2');

if (formins && prev_save) {
    prev_save.addEventListener('click', function () {
            formins.submit();
    });
    prev_save.addEventListener('keydown', function () {
        if (event.keyCode === 13) {
            formins.submit();
        }
    });
}
if (formins && prev_save2) {
    prev_save2.addEventListener('click', function () {
            formins.submit();
    });
    prev_save2.addEventListener('keydown', function () {
        if (event.keyCode === 13) {
            formins.submit();
        }
    });
}

var save_btn = document.getElementById('save-article');
var save_btn2 = document.getElementById('save-article2');
if (formins && save_btn) {
    save_btn.addEventListener('click', function () {
        // formins.submit();
        if (rm) {
            save_html();
        }
    });
    save_btn.addEventListener('keydown', function () {
        if (event.keyCode === 13) {
            // formins.submit();
            save_html();
        }
    });
}
if (formins && save_btn2) {
    save_btn2.addEventListener('click', function () {
        // formins.submit();
        if (rm) {
            save_html();
        }
    });
    save_btn2.addEventListener('keydown', function () {
        if (event.keyCode === 13) {
            // formins.submit();
            save_html();
        }
    });
}

if (formins) {
    
    
    // var md = new Remarkable();
    // console.log(md.render('# Remarkable rulezz!'));
    
    // var rm = new Remarkable();
    
    // Actual default values
    var rm = new Remarkable({
        html:         true,        // Enable HTML tags in source
        xhtmlOut:     false,        // Use '/' to close single tags (<br />)
        breaks:       true,        // Convert '\n' in paragraphs into <br>
        langPrefix:   'language-',  // CSS language prefix for fenced blocks
        linkify:      true,        // Autoconvert URL-like text to links
        
        // Enable some language-neutral replacement + quotes beautification
        typographer:  true,
        
        // Double + single quotes replacement pairs, when typographer enabled,
        // and smartquotes on. Set doubles to '«»' for Russian, '„“' for German.
        quotes: '“”‘’',
        
        // Highlighter function. Should return escaped HTML,
        // or '' if the source string is not changed
        // highlight: function (/*str, lang*/) { return ''; }
        highlight: function (str, lang) {
            if (lang && hljs.getLanguage(lang)) {
                try {
                    return hljs.highlight(lang, str).value;
                } catch (err) {}
            }
            try {
                return hljs.highlightAuto(str).value;
            } catch (err) {}
            
            return ''; // use external default escaping
        }
    });
    // console.log(rm.render('# Remarkable rulezz!'));
  
}


/* Buttons
    Edit Save - submit_markdown()
    Edit Prev - preview_markdown() and submit_markdown()
    
    Prev Save - submit_markdown()
    
*/























