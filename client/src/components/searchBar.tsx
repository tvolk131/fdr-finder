import {
  IconButton,
  InputBase,
  AccordionSummary,
  Accordion,
  AccordionDetails,
  Divider,
  Chip,
  CircularProgress,
  TextField
} from '@mui/material';
import {Autocomplete} from '@mui/lab';
import {Theme} from '@mui/material/styles';
import {createStyles, makeStyles} from '@mui/styles';
import {ExpandMore as ExpandMoreIcon, Close as CloseIcon} from '@mui/icons-material';
import * as React from 'react';
import {MouseEvent, useState} from 'react';
import {getTagDisplayText} from '../helper/tagFormatting';

const useStyles = makeStyles((theme: Theme) => (
  createStyles({
    root: {
      padding: '2px 4px',
      display: 'flex',
      alignItems: 'center'
    },
    autocomplete: {
      marginLeft: 8,
      flex: 1
    },
    inputBaseRoot: {
      width: '100%'
    },
    inputBaseInput: {
      padding: '12px 0'
    },
    iconButton: {
      padding: 10
    },
    verticalDivider: {
      margin: '0 5px'
    },
    tagChip: {
      margin: theme.spacing(0.5)
    },
    tagSearchFieldWrapper: {
      display: 'block',
      textAlign: 'center',
      paddingBottom: theme.spacing(0.75)
    },
    advancedSearchWrapper: {
      width: '100%',
      textAlign: 'center'
    },
    accordionSummaryContent: {
      margin: '8px 0'
    },
    loadingSpinner: {
      marginTop: '12px'
    }
  })
));

const maxVisibleTags = 50;

interface SearchBarProps {
  searchText: string
  setSearchText: (query: string) => void
  searchTags: string[]
  setSearchTags: (tags: string[]) => void
  tagsWithCounts: {tag: string, count: number}[]
  isLoadingTagsWithCounts: boolean
}

const SearchBar = (props: SearchBarProps) => {
  const [tagFilter, setTagFilter] = useState('');

  const handleMouseDownSearch = (event: MouseEvent) => {
    event.preventDefault();
  };

  const classes = useStyles();

  const getSelectableTagChips = () => {
    const filteredTags = props.tagsWithCounts.filter(({tag}) => (
      getTagDisplayText(tag).toLowerCase().includes(tagFilter.toLowerCase())
    ))

    const tagChips = filteredTags.slice(0, maxVisibleTags).map(({tag, count}) => (
      <Chip
        onClick={() => props.setSearchTags([...props.searchTags, tag])}
        label={`${getTagDisplayText(tag)} (${count})`}
        className={classes.tagChip}
      />
    ));

    const nonVisibleTagCount = filteredTags.length - maxVisibleTags;

    if (nonVisibleTagCount > 0) {
      tagChips.push(<Chip
        label={`... +${nonVisibleTagCount}`}
        className={classes.tagChip}
      />);
    }

    return tagChips;
  };

  return (
    <Accordion>
      <AccordionSummary expandIcon={<ExpandMoreIcon/>} classes={{content: classes.accordionSummaryContent}}>
        <div
          onClick={(event) => event.stopPropagation()}
          onFocus={(event) => event.stopPropagation()}
          style={{
            width: '100%',
            display: 'flex'
          }}
        >
          <Autocomplete
            freeSolo
            options={[]} // TODO - Re-enable autocomplete suggestions by setting some state here.
            className={classes.autocomplete}
            inputValue={props.searchText}
            onInputChange={(event, value, reason) => {
              if (!(value.length === 0 && reason === 'reset')) {
                props.setSearchText(value);
              }
            }}
            renderInput={(params: any) => (
              <div ref={params.InputProps.ref}>
                <InputBase
                  classes={{root: classes.inputBaseRoot, input: classes.inputBaseInput}}
                  placeholder='Search Freedomain Videos'
                  {...params.inputProps}
                />
              </div>
            )}
          />
          {!!props.searchText.length && (
            <IconButton
              className={classes.iconButton}
              onMouseDown={handleMouseDownSearch}
              onClick={() => {
                props.setSearchText('');
              }}
            >
              <CloseIcon/>
            </IconButton>
          )}
          {!!props.searchText.length && <Divider className={classes.verticalDivider} orientation={'vertical'}/>}
          {!!props.searchTags.length && props.searchTags.map((tag) => (
            <Chip
              onDelete={() => props.setSearchTags(props.searchTags.filter((iterTag) => tag !== iterTag))}
              className={classes.tagChip}
              label={getTagDisplayText(tag)}
            />
          ))}
        </div>
      </AccordionSummary>
      <Divider/>
      <AccordionDetails>
        <div className={classes.advancedSearchWrapper}>
          <div className={classes.tagSearchFieldWrapper}>
            <TextField value={tagFilter} onChange={(e) => setTagFilter(e.target.value)} label={'Tag Filter'}/>
          </div>
          {props.isLoadingTagsWithCounts ?
            <CircularProgress className={classes.loadingSpinner}/> : getSelectableTagChips()}
        </div>
      </AccordionDetails>
    </Accordion>
  );
};

export default SearchBar;