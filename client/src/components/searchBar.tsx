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
} from '@material-ui/core';
import {Autocomplete} from '@material-ui/lab';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
import {ExpandMore as ExpandMoreIcon, Close as CloseIcon} from '@material-ui/icons';
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
  tagFilter: string
  setTagFilter: (filter: string) => void
  searchTags: string[]
  setSearchTags: (tags: string[]) => void
  tagsWithCounts: {tags: {tag: string, count: number}[], remainingTagCount: number}
  isLoadingTagsWithCounts: boolean
}

const SearchBar = (props: SearchBarProps) => {
  const handleMouseDownSearch = (event: MouseEvent) => {
    event.preventDefault();
  };

  const classes = useStyles();

  const getSelectableTagChips = () => {
    const tagChips = props.tagsWithCounts.tags.map(({tag, count}) => (
      <Chip
        onClick={() => props.setSearchTags([...props.searchTags, tag])}
        className={classes.tagChip}
        label={`${getTagDisplayText(tag)} (${count})`}
      />
    ));

    const nonVisibleTagCount = props.tagsWithCounts.remainingTagCount;

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
            <TextField
              value={props.tagFilter}
              onChange={(e) => props.setTagFilter(e.target.value)} label={'Tag Filter'}
            />
          </div>
          {props.isLoadingTagsWithCounts ?
            <CircularProgress className={classes.loadingSpinner}/> : getSelectableTagChips()}
        </div>
      </AccordionDetails>
    </Accordion>
  );
};

export default SearchBar;